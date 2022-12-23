use std::collections::{HashMap, HashSet};

use crate::utils::read_some_to_vec;

type Facing = usize;

const RIGHT: Facing = 0;
const DOWN: Facing = 1;
const LEFT: Facing = 2;
const UP: Facing = 3;

enum Move {
    Left,
    Right,
    Forward(usize),
}

// TODO clean this up. There has to be a way to make this less hacky.

type Vec3 = [i32; 3];
type Vec2 = [i32; 2];

type Rotation = [[i32; 3]; 3];
const IDENTITY: Rotation = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];

fn add2(lhs: Vec2, rhs: Vec2) -> Vec2 {
    [lhs[0] + rhs[0], lhs[1] + rhs[1]]
}

fn add3(lhs: Vec3, rhs: Vec3) -> Vec3 {
    [lhs[0] + rhs[0], lhs[1] + rhs[1], lhs[2] + rhs[2]]
}

fn sub3(lhs: Vec3, rhs: Vec3) -> Vec3 {
    [lhs[0] - rhs[0], lhs[1] - rhs[1], lhs[2] - rhs[2]]
}

fn dot3(lhs: Vec3, rhs: Vec3) -> i32 {
    lhs[0] * rhs[0] + lhs[1] * rhs[1] + lhs[2] * rhs[2]
}

fn rotate(m: Rotation, v: Vec3) -> Vec3 {
    [
        m[0][0] * v[0] + m[0][1] * v[1] + m[0][2] * v[2],
        m[1][0] * v[0] + m[1][1] * v[1] + m[1][2] * v[2],
        m[2][0] * v[0] + m[2][1] * v[1] + m[2][2] * v[2],
    ]
}

fn matmul(a: Rotation, b: Rotation) -> Rotation {
    let mut result = [[0; 3]; 3];
    for r in 0..3 {
        for c in 0..3 {
            result[r][c] = a[r][0] * b[0][c] + a[r][1] * b[1][c] + a[r][2] * b[2][c];
        }
    }
    result
}

const ROTATIONS: [Rotation; 4] = [
    [[0, 0, 1], [0, 1, 0], [-1, 0, 0]],
    [[1, 0, 0], [0, 0, -1], [0, 1, 0]],
    [[0, 0, -1], [0, 1, 0], [1, 0, 0]],
    [[1, 0, 0], [0, 0, 1], [0, -1, 0]],
];

fn path(faces: &Vec<Face>, lookup: &HashMap<Vec2, usize>, start: usize, end: usize) -> Vec<Facing> {
    let mut visited = HashSet::new();
    let mut came_from = HashMap::new();
    let mut frontier = vec![start];
    while let Some(current) = frontier.pop() {
        if current == end {
            break;
        }

        visited.insert(current);
        for facing in [RIGHT, DOWN, LEFT, UP] {
            let pos = add2(faces[current].position, FORWARD2[facing]);
            if lookup.contains_key(&pos) {
                let next = lookup[&pos];
                if !visited.contains(&next) {
                    came_from.insert(next, (current, facing));
                    frontier.push(next);
                }
            }
        }
    }

    let mut current = end;
    let mut path = vec![];
    while current != start {
        let prev = came_from[&current];
        path.push(prev.1);
        current = prev.0;
    }

    path.reverse();
    path
}

const FORWARD2: [Vec2; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];

struct Face {
    position: Vec2,
    rotation: Rotation,
}

#[derive(Debug)]
struct Tile2 {
    position: Vec2,
    is_wall: bool,
    neighbors: [usize; 4],
}

struct Tile3 {
    face: usize,
    position: Vec3,
    map_pos: Vec2,
    is_wall: bool,
    neighbors: [(usize, Facing); 4],
}

struct State {
    index: usize,
    facing: usize,
}

fn find_neighbor(
    start: Vec2,
    lookup: &HashMap<Vec2, usize>,
    facing: Facing,
    rows: i32,
    columns: i32,
) -> Vec2 {
    let mut cursor = add2(start, FORWARD2[facing]);
    loop {
        if cursor[0] == 0 {
            cursor = [rows, cursor[1]]
        } else if cursor[0] == rows {
            cursor = [1, cursor[1]]
        } else if cursor[1] == 0 {
            cursor = [cursor[0], columns]
        } else if cursor[1] == columns {
            cursor = [cursor[0], 1]
        }

        if lookup.contains_key(&cursor) {
            break;
        }

        cursor = add2(cursor, FORWARD2[facing]);
    }
    cursor
}

fn get_size(lines: &Vec<String>) -> usize {
    let width = lines[0].trim().len();
    for r in 1..lines.len() {
        if lines[r].trim().len() != width {
            return r.min(width);
        }
    }
    panic!()
}

fn read_tiles2(lines: &Vec<String>) -> Vec<Tile2> {
    let mut tiles = (0..lines.len() - 1)
        .flat_map(|r| {
            lines[r]
                .chars()
                .enumerate()
                .filter_map(move |(c, t)| match t {
                    '.' => Some(Tile2 {
                        position: [r as i32 + 1, c as i32 + 1],
                        is_wall: false,
                        neighbors: [0; 4],
                    }),
                    '#' => Some(Tile2 {
                        position: [r as i32 + 1, c as i32 + 1],
                        is_wall: true,
                        neighbors: [0; 4],
                    }),
                    _ => None,
                })
        })
        .collect::<Vec<Tile2>>();
    let rows = tiles.iter().map(|t| t.position[0]).max().unwrap() + 1;
    let columns = tiles.iter().map(|t| t.position[1]).max().unwrap() + 1;
    let lookup = (0..tiles.len()).map(|i| (tiles[i].position, i)).collect();

    for t in tiles.iter_mut() {
        for facing in [RIGHT, DOWN, LEFT, UP] {
            t.neighbors[facing] = lookup[&find_neighbor(t.position, &lookup, facing, rows, columns)]
        }
    }

    tiles
}

fn find_neighbor3(
    tiles: &Vec<Tile3>,
    lookup: &HashMap<Vec3, usize>,
    facings: &[[Vec3; 4]; 6],
    start: usize,
    facing: usize,
    size: i32,
) -> (usize, Facing) {
    let tile = &tiles[start];
    let mut neighbor = add3(tile.position, facings[tile.face][facing]);
    if lookup.contains_key(&neighbor) {
        return (lookup[&neighbor], facing);
    }

    // the direction of the correction is the
    // direction we need to continue in
    let correction = match tile.position {
        [x, _, _] if x == size => [-1, 0, 0],
        [x, _, _] if x == -size => [1, 0, 0],
        [_, y, _] if y == size => [0, -1, 0],
        [_, y, _] if y == -size => [0, 1, 0],
        [_, _, z] if z == size => [0, 0, -1],
        [_, _, z] if z == -size => [0, 0, 1],
        _ => panic!(),
    };

    neighbor = add3(neighbor, correction);

    for i in 0..3 {
        if neighbor[i] < -size {
            neighbor[i] += 1
        } else if neighbor[i] > size {
            neighbor[i] -= 1
        }
    }

    let next = lookup[&neighbor];
    let facing = [RIGHT, DOWN, LEFT, UP]
        .iter()
        .enumerate()
        .filter_map(
            |(i, f)| match dot3(correction, facings[tiles[next].face][*f]) {
                d if d > 0 => Some(i),
                _ => None,
            },
        )
        .next()
        .unwrap();
    (next, facing)
}

fn read_tiles3(lines: &Vec<String>) -> Vec<Tile3> {
    let size = get_size(&lines) as i32;
    let mut face_lookup = HashMap::new();
    let mut faces = vec![];
    let mut tiles = vec![];
    for r in 0..lines.len() - 1 {
        for (c, t) in lines[r].chars().enumerate() {
            if t == ' ' {
                continue;
            }

            let face_pos = [r as i32 / size + 1, c as i32 / size + 1];
            if !face_lookup.contains_key(&face_pos) {
                face_lookup.insert(face_pos, faces.len());
                faces.push(Face {
                    position: face_pos,
                    rotation: IDENTITY,
                })
            }
            let face = face_lookup[&face_pos];
            let rr = (face_pos[0] - 1) * size;
            let cc = (face_pos[1] - 1) * size;
            let position = [
                -(2 * (c as i32 - cc) as i32 - size + 1),
                2 * (r as i32 - rr) as i32 - size + 1,
                size,
            ];
            match t {
                '.' => tiles.push(Tile3 {
                    face,
                    position,
                    map_pos: [r as i32 + 1, c as i32 + 1],
                    is_wall: false,
                    neighbors: [(0, RIGHT); 4],
                }),
                '#' => tiles.push(Tile3 {
                    face,
                    position,
                    map_pos: [r as i32 + 1, c as i32 + 1],
                    is_wall: true,
                    neighbors: [(0, RIGHT); 4],
                }),
                _ => panic!(),
            }
        }
    }

    for i in 1..faces.len() {
        let path = path(&faces, &face_lookup, i, 0);
        for rot in path {
            faces[i].rotation = matmul(ROTATIONS[rot], faces[i].rotation);
        }
    }

    let mut tile_lookup2 = HashMap::new();
    let mut tile_lookup3 = HashMap::new();
    for (i, tile) in tiles.iter_mut().enumerate() {
        tile.position = rotate(faces[tile.face].rotation, tile.position);
        tile_lookup3.insert(tile.position, i);
        tile_lookup2.insert(tile.map_pos, i);
    }

    let mut face_diffs = [[[0, 0, 0]; 4]; 6];
    for i in 0..6 {
        let face_pos = faces[i].position;
        let rr = (face_pos[0] - 1) * size;
        let cc = (face_pos[1] - 1) * size;
        let tile = tiles[tile_lookup2[&[rr + 1, cc + 1]]].position;
        let right = tiles[tile_lookup2[&[rr + 1, cc + 2]]].position;
        let down = tiles[tile_lookup2[&[rr + 2, cc + 1]]].position;
        face_diffs[i][RIGHT] = sub3(right, tile);
        face_diffs[i][DOWN] = sub3(down, tile);
        face_diffs[i][LEFT] = sub3(tile, right);
        face_diffs[i][UP] = sub3(tile, down);
    }

    for i in 0..tiles.len() {
        for facing in [RIGHT, DOWN, LEFT, UP] {
            tiles[i].neighbors[facing] =
                find_neighbor3(&tiles, &tile_lookup3, &face_diffs, i, facing, size)
        }
    }

    tiles
}

fn read_moves(lines: &Vec<String>) -> Vec<Move> {
    let moves = lines
        .last()
        .unwrap()
        .split_inclusive(['R', 'L'])
        .flat_map(|m| {
            match (
                m.chars().take(m.len() - 1).collect::<String>(),
                m.chars().last().unwrap(),
            ) {
                (steps, 'R') => vec![Move::Forward(steps.parse().unwrap()), Move::Right],
                (steps, 'L') => vec![Move::Forward(steps.parse().unwrap()), Move::Left],
                _ => {
                    vec![Move::Forward(m.parse().unwrap())]
                }
            }
        })
        .collect();

    moves
}

fn part1(lines: &Vec<String>) -> i32 {
    let (tiles, moves) = (read_tiles2(lines), read_moves(lines));
    let mut state = State {
        index: 0,
        facing: RIGHT,
    };

    for m in moves {
        match m {
            Move::Left => state.facing = (state.facing + 3) % 4,
            Move::Right => state.facing = (state.facing + 1) % 4,
            Move::Forward(steps) => {
                for _ in 0..steps {
                    let next = tiles[state.index].neighbors[state.facing];
                    if !tiles[next].is_wall {
                        state.index = next
                    }
                }
            }
        }
    }

    let pos = tiles[state.index].position;
    1000 * pos[0] + 4 * pos[1] + state.facing as i32
}

fn part2(lines: &Vec<String>) -> i32 {
    let (tiles, moves) = (read_tiles3(lines), read_moves(lines));
    let mut state = State {
        index: 0,
        facing: RIGHT,
    };

    for m in moves {
        match m {
            Move::Left => state.facing = (state.facing + 3) % 4,
            Move::Right => state.facing = (state.facing + 1) % 4,
            Move::Forward(steps) => {
                for _ in 0..steps {
                    let (next, facing) = tiles[state.index].neighbors[state.facing];
                    if !tiles[next].is_wall {
                        state.index = next;
                        state.facing = facing;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    let pos = tiles[state.index].map_pos;
    1000 * pos[0] + 4 * pos[1] + state.facing as i32
}

pub fn run() {
    let lines = read_some_to_vec("data/day22.txt", |line| match line.trim_end() {
        "" => None,
        contents => Some(contents.to_string()),
    });
    println!("== Day 22 ==");
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

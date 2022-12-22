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

type Vec3 = [i32; 3];
type Vec2 = [i32; 2];

fn add2(lhs: Vec2, rhs: Vec2) -> Vec2 {
    [lhs[0] + rhs[0], lhs[1] + rhs[1]]
}

fn add3(lhs: Vec3, rhs: Vec3) -> Vec3 {
    [lhs[0] + rhs[0], lhs[1] + rhs[1], lhs[2] + rhs[2]]
}

fn path(faces: &HashSet<Vec2>, start: Vec2, end: Vec2) -> Vec<Facing> {
    let mut visited = HashSet::new();
    let mut came_from = HashMap::new();
    let mut frontier = vec![start];
    while let Some(current) = frontier.pop() {
        if current == end {
            break;
        }

        visited.insert(current);
        for facing in [RIGHT, DOWN, LEFT, UP] {
            let next = add2(current, FORWARD2[facing]);
            if faces.contains(&next) && !visited.contains(&next) {
                came_from.insert(next, (current, facing));
                frontier.push(next);
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
    forward: [Vec3; 4],
}

#[derive(Debug)]
struct Tile2 {
    position: Vec2,
    is_wall: bool,
    neighbors: [usize; 4],
}

struct Tile3 {
    face: Vec2,
    position: Vec3,
    is_wall: bool,
    neighbors: [usize; 4],
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

fn read_tiles3(lines: &Vec<String>) -> Vec<Tile3> {
    let size = get_size(&lines) as i32;
    let mut tiles3d = (0..size)
        .flat_map(|r| {
            lines[r as usize]
                .chars()
                .enumerate()
                .filter_map(move |(c, t)| {
                    let face = [r / size + 1, c as i32 / size + 1];
                    let rr = (face[0] - 1) * size;
                    let cc = (face[1] - 1) * size;
                    let position = [(r + 1 - rr) as i32, (c as i32 + 1 - cc) as i32, 1];
                    match t {
                        '.' => Some(Tile3 {
                            face,
                            position,
                            is_wall: false,
                            neighbors: [0; 4],
                        }),
                        '#' => Some(Tile3 {
                            face,
                            position,
                            is_wall: true,
                            neighbors: [0; 4],
                        }),
                        _ => None,
                    }
                })
        })
        .collect::<Vec<Tile3>>();

    let faces = tiles3d.iter().map(|t| t.face).collect::<HashSet<Vec2>>();
    // for each face find the rotation
    // apply rotation to face forward vectors && positions

    tiles3d
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
    0
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

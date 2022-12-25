use std::collections::{HashMap, HashSet};

use crate::maths::{Rot3, Vec2, Vec3};
use crate::utils::{read_some_to_vec, read_to_vec};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Facing {
    Right,
    Down,
    Left,
    Up,
}

enum Move {
    Left,
    Right,
    Forward(usize),
}

impl Facing {
    pub fn iterator() -> impl Iterator<Item = Facing> {
        [Facing::Right, Facing::Down, Facing::Left, Facing::Up]
            .iter()
            .copied()
    }

    fn next(&self) -> Facing {
        match *self {
            Facing::Right => Facing::Down,
            Facing::Down => Facing::Left,
            Facing::Left => Facing::Up,
            Facing::Up => Facing::Right,
        }
    }

    fn prev(&self) -> Facing {
        match *self {
            Facing::Right => Facing::Up,
            Facing::Down => Facing::Right,
            Facing::Left => Facing::Down,
            Facing::Up => Facing::Left,
        }
    }

    fn score(&self) -> i32 {
        match *self {
            Facing::Right => 0,
            Facing::Down => 1,
            Facing::Left => 2,
            Facing::Up => 3,
        }
    }

    fn rot3(self) -> Rot3 {
        let m = match self {
            Facing::Right => [[0, 0, 1], [0, 1, 0], [-1, 0, 0]],
            Facing::Down => [[1, 0, 0], [0, 0, -1], [0, 1, 0]],
            Facing::Left => [[0, 0, -1], [0, 1, 0], [1, 0, 0]],
            Facing::Up => [[1, 0, 0], [0, 0, 1], [0, -1, 0]],
        };
        Rot3 { m }
    }

    fn vec2(self) -> Vec2 {
        let (x, y) = match self {
            Facing::Right => (1, 0),
            Facing::Down => (0, 1),
            Facing::Left => (-1, 0),
            Facing::Up => (0, -1),
        };
        Vec2 { x, y }
    }
}

fn path(faces: &Vec<Face>, lookup: &HashMap<Vec2, usize>, start: usize, end: usize) -> Vec<Facing> {
    let mut visited = HashSet::new();
    let mut came_from = HashMap::new();
    let mut frontier = vec![start];
    while let Some(current) = frontier.pop() {
        if current == end {
            break;
        }

        visited.insert(current);
        for facing in Facing::iterator() {
            let pos = faces[current].map_pos + facing.vec2();
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

struct Face {
    map_pos: Vec2,
    rotation: Rot3,
    facings: HashMap<Facing, Vec3>,
}

struct Tile {
    is_wall: bool,
    map_pos: Vec2,
    neighbors: HashMap<Facing, State>,
}

#[derive(Copy, Clone)]
struct State {
    index: usize,
    facing: Facing,
}

fn read_tiles() -> Vec<Tile> {
    let lines = read_some_to_vec("data/day22.txt", |line| match line.trim_end() {
        "" => None,
        contents => Some(contents.to_string()),
    });
    (0..lines.len() - 1)
        .flat_map(|r| {
            lines[r].chars().enumerate().filter_map(move |(c, t)| {
                let map_pos = Vec2 {
                    x: c as i32 + 1,
                    y: r as i32 + 1,
                };
                match t {
                    '.' => Some(Tile {
                        map_pos,
                        is_wall: false,
                        neighbors: HashMap::new(),
                    }),
                    '#' => Some(Tile {
                        map_pos,
                        is_wall: true,
                        neighbors: HashMap::new(),
                    }),
                    _ => None,
                }
            })
        })
        .collect()
}

fn read_moves() -> Vec<Move> {
    let lines = read_to_vec("data/day22.txt", |line| line.to_string());

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

struct FlatMap {
    lookup: HashMap<Vec2, usize>,
    tiles: Vec<Tile>,
    width: i32,
    height: i32,
}

impl FlatMap {
    fn find_neighbor(&self, start: usize, facing: Facing) -> State {
        let mut cursor = self.tiles[start].map_pos + facing.vec2();
        loop {
            if cursor.x == 0 {
                cursor.x = self.width
            } else if cursor.x == self.width {
                cursor.x = 1
            } else if cursor.y == 0 {
                cursor.y = self.height
            } else if cursor.y == self.height {
                cursor.y = 1
            }

            if self.lookup.contains_key(&cursor) {
                break;
            }

            cursor = cursor + facing.vec2();
        }
        State {
            index: self.lookup[&cursor],
            facing,
        }
    }

    pub fn new() -> FlatMap {
        let tiles = read_tiles();
        let width = tiles.iter().map(|t| t.map_pos.x).max().unwrap() + 1;
        let height = tiles.iter().map(|t| t.map_pos.y).max().unwrap() + 1;
        let lookup = (0..tiles.len()).map(|i| (tiles[i].map_pos, i)).collect();
        let mut map = FlatMap {
            tiles,
            width,
            height,
            lookup,
        };

        for i in 0..map.tiles.len() {
            for facing in Facing::iterator() {
                let next = map.find_neighbor(i, facing);
                map.tiles[i].neighbors.insert(facing, next);
            }
        }

        map
    }
}

struct CubeMap {
    tiles: Vec<Tile>,
    cube_pos: Vec<Vec3>,
    tile_faces: Vec<usize>,
    faces: Vec<Face>,
    cube_lookup: HashMap<Vec3, usize>,
    size: i32,
}

impl CubeMap {
    fn tile_to_cube(tile: &Tile, face: &Face, size: i32) -> Vec3 {
        let map_pos = tile.map_pos;
        let face_pos = face.map_pos;
        let x = (face_pos.x - 1) * size;
        let y = (face_pos.y - 1) * size;
        let pos = Vec3 {
            x: -(2 * (map_pos.x - x - 1) - size + 1),
            y: 2 * (map_pos.y - y - 1) - size + 1,
            z: size,
        };
        face.rotation * pos
    }

    fn facing(&self, tile: usize, facing: Facing) -> Vec3 {
        self.faces[self.tile_faces[tile]].facings[&facing]
    }
    fn find_neighbor(&self, start: usize, facing: Facing) -> State {
        let mut neighbor = self.cube_pos[start] + self.facing(start, facing);
        if self.cube_lookup.contains_key(&neighbor) {
            return State {
                index: self.cube_lookup[&neighbor],
                facing,
            };
        }

        let (x, y, z) = match self.cube_pos[start] {
            Vec3 { x, y: _, z: _ } if x == self.size => (-1, 0, 0),
            Vec3 { x, y: _, z: _ } if x == -self.size => (1, 0, 0),
            Vec3 { x: _, y, z: _ } if y == self.size => (0, -1, 0),
            Vec3 { x: _, y, z: _ } if y == -self.size => (0, 1, 0),
            Vec3 { x: _, y: _, z } if z == self.size => (0, 0, -1),
            Vec3 { x: _, y: _, z } if z == -self.size => (0, 0, 1),
            _ => panic!(),
        };
        let correction = Vec3 { x, y, z };

        neighbor = (neighbor + correction).clip(self.size);

        if !self.cube_lookup.contains_key(&neighbor) {
            println!("error")
        }
        let index = self.cube_lookup[&neighbor];
        let facing = Facing::iterator()
            .find_map(|f| match correction * self.facing(index, f) {
                d if d > 0 => Some(f),
                _ => None,
            })
            .unwrap();
        State { index, facing }
    }

    fn row_width(tiles: &Vec<Tile>, y: i32) -> i32 {
        let x_vals = tiles
            .iter()
            .filter_map(|t| {
                if t.map_pos.y == y {
                    Some(t.map_pos.x)
                } else {
                    None
                }
            })
            .collect::<Vec<i32>>();
        let x_min = x_vals.iter().min().unwrap();
        let x_max = x_vals.iter().max().unwrap();
        x_max - x_min + 1
    }

    fn get_size(tiles: &Vec<Tile>) -> i32 {
        let height = tiles.iter().map(|t| t.map_pos.y).max().unwrap();
        let width = CubeMap::row_width(tiles, 1);
        for y in 1..height {
            let test = CubeMap::row_width(tiles, y);
            if test != width {
                return (y - 1).min(width);
            }
        }
        panic!()
    }

    fn new() -> CubeMap {
        let tiles = read_tiles();
        let size = CubeMap::get_size(&tiles);
        let mut faces = vec![];
        let mut face_lookup = HashMap::new();
        let mut tile_faces = vec![];
        for t in tiles.iter() {
            let face_pos = (t.map_pos - 1) / size + 1;
            if !face_lookup.contains_key(&face_pos) {
                face_lookup.insert(face_pos, faces.len());
                faces.push(Face {
                    map_pos: face_pos,
                    rotation: Rot3::identity(),
                    facings: HashMap::new(),
                })
            }
            tile_faces.push(face_lookup[&face_pos])
        }

        for i in 1..faces.len() {
            let path = path(&faces, &face_lookup, i, 0);
            for facing in path {
                faces[i].rotation = facing.rot3() * faces[i].rotation;
            }
        }

        let cube_pos = tiles
            .iter()
            .zip(tile_faces.iter())
            .map(|(t, f)| CubeMap::tile_to_cube(t, &faces[*f], size))
            .collect::<Vec<Vec3>>();
        let map_lookup = (0..tiles.len())
            .map(|i| (tiles[i].map_pos, i))
            .collect::<HashMap<Vec2, usize>>();
        let cube_lookup = (0..tiles.len()).map(|i| (cube_pos[i], i)).collect();

        for f in faces.iter_mut() {
            let face_pos = f.map_pos;
            let pos = (face_pos - 1) * size + 1;
            let tile = cube_pos[map_lookup[&pos]];
            let right = cube_pos[map_lookup[&(pos + Facing::Right.vec2())]];
            let down = cube_pos[map_lookup[&(pos + Facing::Down.vec2())]];
            f.facings.insert(Facing::Right, right - tile);
            f.facings.insert(Facing::Down, down - tile);
            f.facings.insert(Facing::Left, tile - right);
            f.facings.insert(Facing::Up, tile - down);
        }

        let mut map = CubeMap {
            tiles,
            tile_faces,
            cube_pos,
            cube_lookup,
            faces,
            size,
        };

        for i in 0..map.tiles.len() {
            for facing in Facing::iterator() {
                let neighbor = map.find_neighbor(i, facing);
                map.tiles[i].neighbors.insert(facing, neighbor);
            }
        }

        map
    }
}

trait Map {
    fn tile(&self, index: usize) -> &Tile;

    fn navigate(&self, moves: &Vec<Move>) -> i32 {
        let mut state = State {
            index: 0,
            facing: Facing::Right,
        };

        for m in moves {
            match m {
                Move::Left => state.facing = state.facing.prev(),
                Move::Right => state.facing = state.facing.next(),
                Move::Forward(steps) => {
                    for _ in 0..*steps {
                        let next = self.tile(state.index).neighbors[&state.facing];
                        if self.tile(next.index).is_wall {
                            break;
                        }

                        state = next
                    }
                }
            }
        }

        let pos = self.tile(state.index).map_pos;
        pos.y * 1000 + pos.x * 4 + state.facing.score()
    }
}

impl Map for FlatMap {
    fn tile(&self, index: usize) -> &Tile {
        &self.tiles[index]
    }
}
impl Map for CubeMap {
    fn tile(&self, index: usize) -> &Tile {
        &self.tiles[index]
    }
}

fn part1(moves: &Vec<Move>) -> i32 {
    FlatMap::new().navigate(moves)
}

fn part2(moves: &Vec<Move>) -> i32 {
    CubeMap::new().navigate(moves)
}

pub fn run() {
    let moves = read_moves();
    println!("== Day 22 ==");
    println!("Part 1: {}", part1(&moves));
    println!("Part 2: {}", part2(&moves));
}

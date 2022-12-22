use std::collections::HashMap;

use crate::utils::read_some_to_vec;

const RIGHT: usize = 0;
const DOWN: usize = 1;
const LEFT: usize = 2;
const UP: usize = 3;

enum Move {
    Left,
    Right,
    Forward(usize),
}

type Position = (i32, i32);

struct Tile {
    position: Position,
    is_wall: bool,
    neighbors: [usize; 4],
}

struct State {
    index: usize,
    facing: usize,
}

fn find_neighbor(
    start: Position,
    lookup: &HashMap<Position, usize>,
    move_next: fn(Position) -> Position,
    rows: i32,
    columns: i32,
) -> Position {
    let mut cursor = move_next(start);
    while !lookup.contains_key(&cursor) {
        cursor = move_next(cursor);
        if cursor.0 < 0 {
            cursor = (rows, cursor.1)
        } else if cursor.0 == rows {
            cursor = (0, cursor.1)
        } else if cursor.1 < 0 {
            cursor = (cursor.0, columns)
        } else if cursor.1 == columns {
            cursor = (cursor.0, 0)
        }
    }
    cursor
}

fn read_puzzle() -> (Vec<Tile>, Vec<Move>) {
    let lines = read_some_to_vec("data/day22.txt", |line| match line.trim_end() {
        "" => None,
        contents => Some(contents.to_string()),
    });

    let rows = lines.len() as i32 - 1;
    let columns = lines[..rows as usize]
        .iter()
        .map(|line| line.len() as i32)
        .max()
        .unwrap();

    let mut tiles = (0..rows)
        .flat_map(|r| {
            lines[r as usize]
                .chars()
                .enumerate()
                .filter_map(move |(c, t)| match t {
                    '.' => Some(Tile {
                        position: (r, c as i32),
                        is_wall: false,
                        neighbors: [0; 4],
                    }),
                    '#' => Some(Tile {
                        position: (r, c as i32),
                        is_wall: true,
                        neighbors: [0; 4],
                    }),
                    _ => None,
                })
        })
        .collect::<Vec<Tile>>();

    let lookup = (0..tiles.len())
        .map(|i| (tiles[i].position, i))
        .collect::<HashMap<(i32, i32), usize>>();

    for t in tiles.iter_mut() {
        t.neighbors[UP] =
            lookup[&find_neighbor(t.position, &lookup, |p| (p.0 - 1, p.1), rows, columns)];
        t.neighbors[DOWN] =
            lookup[&find_neighbor(t.position, &lookup, |p| (p.0 + 1, p.1), rows, columns)];
        t.neighbors[LEFT] =
            lookup[&find_neighbor(t.position, &lookup, |p| (p.0, p.1 - 1), rows, columns)];
        t.neighbors[RIGHT] =
            lookup[&find_neighbor(t.position, &lookup, |p| (p.0, p.1 + 1), rows, columns)];
    }

    let moves = lines[rows as usize]
        .split_inclusive(['R', 'L'])
        .map(|m| match m {
            "R" => Move::Right,
            "L" => Move::Left,
            steps => Move::Forward(steps.parse().unwrap()),
        })
        .collect();

    (tiles, moves)
}

fn part1(tiles: &Vec<Tile>, moves: &Vec<Move>) -> i32 {
    let mut state = State {
        index: 0,
        facing: RIGHT,
    };

    for m in moves {
        match m {
            Move::Left => state.facing = (state.facing + 3) % 4,
            Move::Right => state.facing = (state.facing + 1) % 4,
            Move::Forward(steps) => {
                for _ in 0..*steps {
                    let next = tiles[state.index].neighbors[state.facing];
                    if !tiles[next].is_wall {
                        state.index = next
                    }
                }
            }
        }
    }

    let pos = tiles[state.index].position;
    1000 * pos.0 + 4 * pos.1 + state.facing as i32
}

pub fn run() {
    let (tiles, moves) = read_puzzle();
    println!("== Day 22 ==");
    println!("Part 1: {}", part1(&tiles, &moves))
}

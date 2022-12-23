use std::collections::{HashMap, HashSet};

use crate::utils::read_to_vec;

type Point = [i64; 2];

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Elf {
    pos: Point,
}

#[derive(Copy, Clone)]
enum Action {
    Move(Point),
    Wait,
}

type Direction = usize;

const N: Direction = 0;
const S: Direction = 1;
const W: Direction = 2;
const E: Direction = 3;
const NW: Direction = 4;
const NE: Direction = 5;
const SW: Direction = 6;
const SE: Direction = 7;

fn add(lhs: Point, rhs: Point) -> Point {
    [lhs[0] + rhs[0], lhs[1] + rhs[1]]
}

const MOVES: [Point; 8] = [
    [0, -1],
    [0, 1],
    [-1, 0],
    [1, 0],
    [-1, -1],
    [1, -1],
    [-1, 1],
    [1, 1],
];

impl Elf {
    fn propose_move(&self, elves: &HashSet<Point>, start: Direction) -> Action {
        let mut moves = [[0, 0]; 8];
        let mut occupied = [false; 8];
        for i in 0..8 {
            moves[i] = add(self.pos, MOVES[i]);
            occupied[i] = elves.contains(&moves[i]);
        }

        if !occupied.iter().any(|o| *o) {
            return Action::Wait;
        }

        for i in 0..4 {
            let mode = (start + i) % 4;
            let maybe_pos = match mode {
                N if !(occupied[NW] || occupied[N] || occupied[NE]) => Some(moves[N]),
                S if !(occupied[SW] || occupied[S] || occupied[SE]) => Some(moves[S]),
                W if !(occupied[NW] || occupied[W] || occupied[SW]) => Some(moves[W]),
                E if !(occupied[NE] || occupied[E] || occupied[SE]) => Some(moves[E]),
                _ => None,
            };
            if let Some(pos) = maybe_pos {
                return Action::Move(pos);
            }
        }

        Action::Wait
    }
}

fn read_elves() -> Vec<Elf> {
    let lines = read_to_vec("data/day23.txt", |s| s.to_string());
    (0..lines.len())
        .flat_map(|y| {
            lines[y]
                .chars()
                .enumerate()
                .filter_map(move |(x, c)| match c {
                    '#' => Some(Elf {
                        pos: [x as i64, y as i64],
                    }),
                    _ => None,
                })
        })
        .collect()
}

fn update(elves: &Vec<Elf>, stage: Direction) -> (Vec<Elf>, bool) {
    let mut counts = HashMap::new();
    let occupied = elves.iter().map(|e| e.pos).collect();
    let proposed = elves
        .iter()
        .map(|e| e.propose_move(&occupied, stage))
        .collect::<Vec<Action>>();
    for pos in &proposed {
        if let Action::Move(newp) = pos {
            if counts.contains_key(newp) {
                counts.insert(newp, counts[newp] + 1);
            } else {
                counts.insert(newp, 1);
            }
        }
    }

    let actions = proposed
        .iter()
        .map(|a| match a {
            Action::Move(pos) => {
                if counts[pos] > 1 {
                    Action::Wait
                } else {
                    *a
                }
            }
            _ => *a,
        })
        .collect::<Vec<Action>>();
    let update = actions
        .iter()
        .zip(elves.iter())
        .map(|(a, e)| match a {
            Action::Move(newp) => Elf { pos: *newp },
            Action::Wait => *e,
        })
        .collect();
    let moved = proposed.iter().any(|a| match a {
        Action::Move(_) => true,
        _ => false,
    });

    (update, moved)
}

fn count_empty(elves: &Vec<Elf>) -> i64 {
    let x_min = elves.iter().map(|e| e.pos[0]).min().unwrap();
    let y_min = elves.iter().map(|e| e.pos[1]).min().unwrap();
    let x_max = elves.iter().map(|e| e.pos[0]).max().unwrap() + 1;
    let y_max = elves.iter().map(|e| e.pos[1]).max().unwrap() + 1;
    (x_max - x_min) * (y_max - y_min) - elves.len() as i64
}

fn part1(start: &Vec<Elf>) -> i64 {
    let mut elves = start.iter().copied().collect();
    let mut stage = N;
    for _ in 0..10 {
        let (next, moved) = update(&elves, stage);
        if !moved {
            break;
        }

        elves = next;
        stage = (stage + 1) % 4;
    }

    count_empty(&elves)
}

fn part2(start: &Vec<Elf>) -> usize {
    let mut elves = start.iter().copied().collect();
    let mut stage = N;
    let mut step = 1;
    loop {
        let (next, moved) = update(&elves, stage);
        if !moved {
            break;
        }

        elves = next;
        stage = (stage + 1) % 4;
        step += 1
    }

    step
}

pub fn run() {
    let elves = read_elves();
    println!("== Day 23 ==");
    println!("Part 1: {}", part1(&elves));
    println!("Part 2: {}", part2(&elves))
}

use std::collections::{HashMap, HashSet};

use crate::{maths::Vec2, utils::read_to_vec};

#[derive(Copy, Clone)]
enum Action {
    Move(Vec2),
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

const MOVES: [Vec2; 8] = [
    Vec2 { x: 0, y: -1 },
    Vec2 { x: 0, y: 1 },
    Vec2 { x: -1, y: 0 },
    Vec2 { x: 1, y: 0 },
    Vec2 { x: -1, y: -1 },
    Vec2 { x: 1, y: -1 },
    Vec2 { x: -1, y: 1 },
    Vec2 { x: 1, y: 1 },
];

trait Elf<T = Self> {
    fn propose_move(&self, elves: &HashSet<T>, start: Direction) -> Action;
}

impl Elf for Vec2 {
    fn propose_move(&self, elves: &HashSet<Vec2>, start: Direction) -> Action {
        let mut moves = [Vec2::from(0, 0); 8];
        let mut occupied = [false; 8];
        let mut any_occupied = false;
        for i in 0..8 {
            moves[i] = self + &MOVES[i];
            occupied[i] = elves.contains(&moves[i]);
            any_occupied = any_occupied || occupied[i];
        }

        if !any_occupied {
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

fn read_elves() -> Vec<Vec2> {
    let lines = read_to_vec("data/day23.txt", |s| s.to_string());
    (0..lines.len())
        .flat_map(|y| {
            lines[y]
                .chars()
                .enumerate()
                .filter_map(move |(x, c)| match c {
                    '#' => Some(Vec2::from(x as i32, y as i32)),
                    _ => None,
                })
        })
        .collect()
}

fn update(elves: &mut Vec<Vec2>, stage: Direction) -> bool {
    let mut is_proposed = HashMap::new();
    let occupied = elves.iter().copied().collect();
    let mut moved = 0;
    for i in 0..elves.len() {
        match elves[i].propose_move(&occupied, stage) {
            Action::Move(newp) => {
                if let Some((elf, oldp)) = is_proposed.insert(newp, (i, elves[i])) {
                    if elves[elf] != oldp {
                        elves[elf] = oldp;
                        moved -= 1;
                    }
                } else {
                    elves[i] = newp;
                    moved += 1;
                }
            }
            _ => (),
        }
    }

    moved > 0
}

fn count_empty(elves: &Vec<Vec2>) -> i32 {
    let x_min = elves.iter().map(|e| e.x).min().unwrap();
    let y_min = elves.iter().map(|e| e.y).min().unwrap();
    let x_max = elves.iter().map(|e| e.x).max().unwrap() + 1;
    let y_max = elves.iter().map(|e| e.y).max().unwrap() + 1;
    (x_max - x_min) * (y_max - y_min) - elves.len() as i32
}

fn part1(start: &Vec<Vec2>) -> i32 {
    let mut elves = start.iter().copied().collect();
    let mut stage = N;
    for _ in 0..10 {
        let moved = update(&mut elves, stage);
        if !moved {
            break;
        }

        stage = (stage + 1) % 4;
    }

    count_empty(&elves)
}

fn part2(start: &Vec<Vec2>) -> usize {
    let mut elves = start.iter().copied().collect();
    let mut stage = N;
    let mut step = 1;
    loop {
        let moved = update(&mut elves, stage);
        if !moved {
            break;
        }

        stage = (stage + 1) % 4;
        step += 1
    }

    step
}

pub fn run() {
    let elves = read_elves();
    println!("== Day 23 ==");
    println!("Part 1: {}", part1(&elves));
    println!("Part 2: {}", part2(&elves));
}

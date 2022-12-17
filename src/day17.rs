use std::collections::HashSet;
use std::fs::read_to_string;

use crate::utils::{find_next, is_match};

type Point = (i32, i32);
type Rock = Vec<Point>;
struct Room {
    rocks: HashSet<Point>,
    height: i32,
    jet_index: usize,
    rock_index: usize,
    jets: Vec<Jet>,
}

#[derive(Copy, Clone)]
enum RockType {
    HLine,
    Cross,
    Angle,
    VLine,
    Square,
}

enum Jet {
    Left,
    Right,
}

fn add(lhs: &Point, rhs: &Point) -> Point {
    (lhs.0 + rhs.0, lhs.1 + rhs.1)
}

fn create_rock(rock_type: RockType, height: i32) -> Rock {
    let init: Point = (0, height);
    match rock_type {
        RockType::HLine => vec![(2, 3), (3, 3), (4, 3), (5, 3)],
        RockType::Cross => vec![(3, 3), (2, 4), (3, 4), (4, 4), (3, 5)],
        RockType::Angle => vec![(2, 3), (3, 3), (4, 3), (4, 4), (4, 5)],
        RockType::VLine => vec![(2, 3), (2, 4), (2, 5), (2, 6)],
        RockType::Square => vec![(2, 3), (3, 3), (2, 4), (3, 4)],
    }
    .iter()
    .map(|p| add(p, &init))
    .collect()
}

const DOWN: Point = (0, -1);
const LEFT: Point = (-1, 0);
const RIGHT: Point = (1, 0);
const ROCK_ORDER: [RockType; 5] = [
    RockType::HLine,
    RockType::Cross,
    RockType::Angle,
    RockType::VLine,
    RockType::Square,
];

fn move_rock(rock: &mut Rock, dir: &Point) {
    for p in rock {
        *p = add(p, dir)
    }
}

impl Room {
    fn can_move(&self, rock: &Rock, dir: &Point) -> bool {
        for p in rock {
            let q = add(p, dir);
            if match (q, self.rocks.contains(&q)) {
                ((-1, _), _) => true,
                ((7, _), _) => true,
                ((_, -1), _) => true,
                (_, true) => true,
                _ => false,
            } {
                return false;
            }
        }
        true
    }

    fn new() -> Room {
        Room {
            height: 0,
            jets: parse_jets(),
            rock_index: 0,
            jet_index: 0,
            rocks: HashSet::new(),
        }
    }

    fn settle(&mut self, rock: &Rock) {
        for p in rock {
            if !self.rocks.insert(*p) {
                panic!("collision")
            }
            self.height = self.height.max(p.1 + 1);
        }
    }

    fn add_rock(&mut self) {
        let rock_type = ROCK_ORDER[self.rock_index % ROCK_ORDER.len()];
        self.rock_index += 1;
        let mut rock = create_rock(rock_type, self.height);
        loop {
            let jet = match self.jets[self.jet_index] {
                Jet::Left => &LEFT,
                Jet::Right => &RIGHT,
            };
            self.jet_index += 1;
            if self.jet_index == self.jets.len() {
                self.jet_index = 0;
            }
            if self.can_move(&rock, jet) {
                move_rock(&mut rock, jet)
            }
            if self.can_move(&rock, &DOWN) {
                move_rock(&mut rock, &DOWN)
            } else {
                self.settle(&rock);
                break;
            }
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        (0..self.height)
            .into_iter()
            .map(|y| {
                (0..7)
                    .into_iter()
                    .map(|x| {
                        if self.rocks.contains(&(x, y)) {
                            1u8 << x
                        } else {
                            0u8
                        }
                    })
                    .reduce(|a, b| a | b)
                    .unwrap()
            })
            .collect()
    }
}

fn parse_jets() -> Vec<Jet> {
    match read_to_string("data/day17.txt") {
        Ok(contents) => contents
            .trim()
            .chars()
            .map(|c| match c {
                '<' => Jet::Left,
                '>' => Jet::Right,
                _ => panic!("Invalid input"),
            })
            .collect(),
        _ => vec![],
    }
}

fn simulate_rocks(steps: usize) -> Room {
    let mut room = Room::new();

    for _ in 0..steps {
        room.add_rock()
    }

    room
}

fn part1() -> i32 {
    simulate_rocks(2022).height
}

fn part2() -> i64 {
    let levels = simulate_rocks(5000).to_bytes();
    const WINDOW_SIZE: usize = 10;
    let mut pattern_start = 0i32;
    let mut pattern_length = 0i32;
    for i in 0..levels.len() {
        if let Some(found) = find_next(&levels, i, i + WINDOW_SIZE) {
            if is_match(&levels, i, found, found - i) {
                pattern_start = i as i32;
                pattern_length = (found - i) as i32;
                break;
            }
        }
    }

    let mut room = Room::new();

    let mut sequence = vec![];
    let mut sequence_start = 0;
    while room.height - pattern_start - pattern_length < pattern_length {
        room.add_rock();
        let index = room.height - pattern_start - pattern_length;
        if index >= 0 {
            if sequence.len() == 0 {
                sequence_start = room.rock_index;
            }
            sequence.push(room.height)
        }
    }

    sequence.pop();
    let sequence_length = sequence.len();
    let tail_length = (1000000000000i64 - sequence_start as i64) % sequence_length as i64;
    let num_sequences = (1000000000000i64 - sequence_start as i64) / sequence_length as i64;
    sequence[tail_length as usize - 1] as i64 + pattern_length as i64 * num_sequences
}

pub fn run() {
    println!("== Day 17 ==");
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

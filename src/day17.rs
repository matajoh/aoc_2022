use std::collections::HashSet;
use std::fs::read_to_string;

use crate::{
    maths::Vec2,
    utils::{find_next, is_match},
};

type Rock = Vec<Vec2>;
struct Room {
    rocks: HashSet<Vec2>,
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

fn create_rock(rock_type: RockType, height: i32) -> Rock {
    let init = Vec2::from(0, height);
    match rock_type {
        RockType::HLine => vec![
            Vec2::from(2, 3),
            Vec2::from(3, 3),
            Vec2::from(4, 3),
            Vec2::from(5, 3),
        ],
        RockType::Cross => vec![
            Vec2::from(3, 3),
            Vec2::from(2, 4),
            Vec2::from(3, 4),
            Vec2::from(4, 4),
            Vec2::from(3, 5),
        ],
        RockType::Angle => vec![
            Vec2::from(2, 3),
            Vec2::from(3, 3),
            Vec2::from(4, 3),
            Vec2::from(4, 4),
            Vec2::from(4, 5),
        ],
        RockType::VLine => vec![
            Vec2::from(2, 3),
            Vec2::from(2, 4),
            Vec2::from(2, 5),
            Vec2::from(2, 6),
        ],
        RockType::Square => vec![
            Vec2::from(2, 3),
            Vec2::from(3, 3),
            Vec2::from(2, 4),
            Vec2::from(3, 4),
        ],
    }
    .iter()
    .map(|p| p + &init)
    .collect()
}

const DOWN: Vec2 = Vec2 { x: 0, y: -1 };
const LEFT: Vec2 = Vec2 { x: -1, y: 0 };
const RIGHT: Vec2 = Vec2 { x: 1, y: 0 };
const ROCK_ORDER: [RockType; 5] = [
    RockType::HLine,
    RockType::Cross,
    RockType::Angle,
    RockType::VLine,
    RockType::Square,
];

fn move_rock(rock: &mut Rock, dir: Vec2) {
    for p in rock {
        *p = *p + dir
    }
}

impl Room {
    fn can_move(&self, rock: &Rock, dir: &Vec2) -> bool {
        for p in rock {
            let q = p + dir;
            if match (q, self.rocks.contains(&q)) {
                (Vec2 { x: -1, y: _ }, _) => true,
                (Vec2 { x: 7, y: _ }, _) => true,
                (Vec2 { x: _, y: -1 }, _) => true,
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
            self.height = self.height.max(p.y + 1);
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
                move_rock(&mut rock, *jet)
            }
            if self.can_move(&rock, &DOWN) {
                move_rock(&mut rock, DOWN)
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
                        if self.rocks.contains(&Vec2 { x, y }) {
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
    sequence[tail_length as usize] as i64 + pattern_length as i64 * num_sequences
}

pub fn run() {
    println!("== Day 17 ==");
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

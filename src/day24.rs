use std::collections::HashSet;

use crate::{
    maths::Vec2,
    utils::{astar_search_mut, read_to_vec, SearchInfoMut},
};

#[derive(Copy, Clone)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

struct Blizzard {
    start: Vec2,
    direction: Direction,
}

impl Blizzard {
    fn from(x: usize, y: usize, direction: Direction) -> Blizzard {
        Blizzard {
            start: Vec2::from(x as i32 - 1, y as i32 - 1),
            direction,
        }
    }

    fn at(&self, minute: usize, width: i32, height: i32) -> Vec2 {
        let steps = minute as i32;
        let (x, y) = match (self.start, self.direction) {
            (Vec2 { x, y }, Direction::Right) => ((x + steps).rem_euclid(width), y),
            (Vec2 { x, y }, Direction::Down) => (x, (y + steps).rem_euclid(height)),
            (Vec2 { x, y }, Direction::Left) => ((x - steps).rem_euclid(width), y),
            (Vec2 { x, y }, Direction::Up) => (x, (y - steps).rem_euclid(height)),
        };
        Vec2 { x, y }
    }
}

struct Map {
    start: State,
    goal: Vec2,
    blizzards: Vec<Blizzard>,
    width: i32,
    height: i32,
    blizzards_memo: Vec<HashSet<Vec2>>,
}

fn to_tiles(line: &str) -> Vec<char> {
    line.trim().chars().collect()
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct State {
    pos: Vec2,
    minute: usize,
}

impl State {
    fn next(&self, pos: Vec2) -> State {
        State {
            pos,
            minute: self.minute + 1,
        }
    }
}

impl Map {
    fn new() -> Map {
        let tiles = read_to_vec("data/day24.txt", to_tiles);
        let width = (tiles[0].len() - 2) as i32;
        let height = (tiles.len() - 2) as i32;
        let blizzards = tiles
            .into_iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.into_iter()
                    .enumerate()
                    .filter_map(move |(x, t)| match t {
                        '>' => Some(Blizzard::from(x, y, Direction::Right)),
                        'v' => Some(Blizzard::from(x, y, Direction::Down)),
                        '<' => Some(Blizzard::from(x, y, Direction::Left)),
                        '^' => Some(Blizzard::from(x, y, Direction::Up)),
                        _ => None,
                    })
            })
            .collect();
        Map {
            start: State {
                pos: Vec2::from(0, -1),
                minute: 0,
            },
            goal: Vec2::from(width - 1, height),
            blizzards,
            width,
            height,
            blizzards_memo: vec![],
        }
    }

    fn is_wall(&self, pos: Vec2) -> bool {
        if pos.x == -1 {
            true
        } else if pos.x == self.width {
            true
        } else if pos.y == -1 && pos.x > 0 {
            true
        } else if pos.y == self.height && pos.x < self.width - 1 {
            true
        } else {
            false
        }
    }

    fn blizzard_at(&mut self, pos: &Vec2, minute: usize) -> bool {
        for m in self.blizzards_memo.len()..=minute {
            self.blizzards_memo.push(
                self.blizzards
                    .iter()
                    .map(|b| b.at(m, self.width, self.height))
                    .collect(),
            );
        }

        self.blizzards_memo[minute].contains(pos)
    }

    fn is_open(&mut self, state: &State) -> bool {
        if self.is_wall(state.pos) {
            false
        } else if self.blizzard_at(&state.pos, state.minute) {
            false
        } else {
            true
        }
    }
}

impl SearchInfoMut<State, usize> for Map {
    fn neighbors(&mut self, node: &State) -> Vec<State> {
        [
            node.pos,
            node.pos + Vec2::from(1, 0),
            node.pos + Vec2::from(0, 1),
            node.pos + Vec2::from(-1, 0),
            node.pos + Vec2::from(0, -1),
        ]
        .into_iter()
        .filter_map(|p| match node.next(p) {
            next if self.is_open(&next) => Some(next),
            _ => None,
        })
        .collect()
    }

    fn heuristic(&self, node: &State) -> usize {
        (node.pos - self.goal).len()
    }

    fn distance(&self, start: &State, end: &State) -> usize {
        end.minute - start.minute
    }

    fn start(&self) -> State {
        self.start
    }

    fn is_goal(&self, node: &State) -> bool {
        node.pos == self.goal
    }

    fn zero() -> usize {
        0
    }

    fn infinity() -> usize {
        usize::MAX
    }
}

fn part1() -> usize {
    let mut map = Map::new();
    let (_, state) = astar_search_mut(&mut map).unwrap();
    state.minute
}

fn part2() -> usize {
    let mut map = Map::new();
    let (_, state0) = astar_search_mut(&mut map).unwrap();
    map.start = state0;
    map.goal = Vec2::from(0, -1);
    let (_, state1) = astar_search_mut(&mut map).unwrap();
    map.start = state1;
    map.goal = Vec2::from(map.width - 1, map.height);
    let (_, state2) = astar_search_mut(&mut map).unwrap();
    state2.minute
}

pub fn run() {
    println!("== Day 24 ==");
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2())
}

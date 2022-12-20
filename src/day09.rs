use crate::utils::read_some_to_vec;
use std::collections::HashSet;

enum Move {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

type Knot = (i32, i32);

fn to_move(line: &str) -> Option<Move> {
    let parts: Vec<&str> = line.trim().split(" ").collect();
    let count = parts[1].parse::<usize>();
    match (parts[0], count) {
        ("U", Ok(units)) => Some(Move::Up(units)),
        ("D", Ok(units)) => Some(Move::Down(units)),
        ("L", Ok(units)) => Some(Move::Left(units)),
        ("R", Ok(units)) => Some(Move::Right(units)),
        _ => None,
    }
}

fn follow(head: Knot, tail: Knot) -> Knot {
    match (head.0 - tail.0, head.1 - tail.1) {
        (dx, dy) if dx.abs() <= 1 && dy.abs() <= 1 => tail,
        (-2, 0) => (tail.0 - 1, tail.1),
        (0, -2) => (tail.0, tail.1 - 1),
        (2, 0) => (tail.0 + 1, tail.1),
        (0, 2) => (tail.0, tail.1 + 1),
        (-2, -1) | (-1, -2) | (-2, -2) => (tail.0 - 1, tail.1 - 1),
        (-2, 1) | (-1, 2) | (-2, 2) => (tail.0 - 1, tail.1 + 1),
        (2, -1) | (1, -2) | (2, -2) => (tail.0 + 1, tail.1 - 1),
        (2, 1) | (1, 2) | (2, 2) => (tail.0 + 1, tail.1 + 1),
        (_, _) => panic!("invalid head/tail position"),
    }
}

fn do_moves(
    visited: &mut HashSet<Knot>,
    rope: &mut Vec<Knot>,
    units: usize,
    update: fn(Knot) -> Knot,
) {
    for _ in 0..units {
        rope[0] = update(rope[0]);
        for i in 1..rope.len() {
            rope[i] = follow(rope[i - 1], rope[i])
        }
        visited.insert(*rope.last().unwrap());
    }
}

fn move_rope(moves: &Vec<Move>, length: usize) -> usize {
    let mut rope = vec![(0, 0); length];
    let mut visited = HashSet::new();
    for m in moves {
        match *m {
            Move::Up(units) => do_moves(&mut visited, &mut rope, units, |(r, c)| (r - 1, c)),
            Move::Down(units) => do_moves(&mut visited, &mut rope, units, |(r, c)| (r + 1, c)),
            Move::Left(units) => do_moves(&mut visited, &mut rope, units, |(r, c)| (r, c - 1)),
            Move::Right(units) => do_moves(&mut visited, &mut rope, units, |(r, c)| (r, c + 1)),
        }
    }
    visited.len()
}

pub fn run() {
    let moves = read_some_to_vec("data/day09.txt", to_move);
    println!("== Day 09 ==");
    println!("Part 1: {}", move_rope(&moves, 2));
    println!("Part 2: {}", move_rope(&moves, 10))
}

use super::utils::read_to_vec;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

type Cell = (usize, usize);

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    position: Cell,
    cost: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const MAX: usize = 25;
const MIN: usize = 0;
const INFINITY: usize = usize::MAX;

#[derive(Copy, Clone, Debug)]
enum Square {
    Start,
    End,
    Ground(usize),
}

impl Square {
    fn can_reach(&self, other: &Square) -> bool {
        match [*other, *self] {
            [Square::Start, Square::Ground(height)] => height <= MIN + 1,
            [_, Square::Start] => true,
            [Square::Ground(h0), Square::End] => MAX <= h0 + 1,
            [Square::Ground(h0), Square::Ground(h1)] => h1 <= h0 + 1,
            [Square::End, _] => true,
            [Square::Start, Square::End] => false,
        }
    }
}

struct Map {
    squares: Vec<Vec<Square>>,
    rows: usize,
    cols: usize,
    start: Cell,
    end: Cell,
}

impl Map {
    fn to_neighbor(&self, square: Cell, other: Cell) -> Option<Cell> {
        let neighbor = match [square, other] {
            [(r0, c0), (r1, c1)] if self.squares[r0][c0].can_reach(&self.squares[r1][c1]) => {
                Some(other)
            }
            _ => None,
        };
        neighbor
    }
    fn neighbors(&self, square: Cell) -> Vec<Cell> {
        let r = square.0 as i32;
        let c = square.1 as i32;
        [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)]
            .map(|c| match c {
                (-1, _) | (_, -1) => None,
                (r, _) if r as usize == self.rows => None,
                (_, c) if c as usize == self.cols => None,
                (r, c) => Some((r as usize, c as usize)),
            })
            .into_iter()
            .filter_map(|c| c)
            .filter_map(|c| self.to_neighbor(square, c))
            .collect()
    }
}

fn to_squares(line: &str) -> Vec<Square> {
    const BASE: usize = 'a' as usize;
    line.trim()
        .chars()
        .map(|c| match c {
            'S' => Square::Start,
            'E' => Square::End,
            _ => Square::Ground((c as usize) - BASE),
        })
        .collect()
}

fn find_terminal(squares: &Vec<Vec<Square>>, square: Square) -> (usize, usize) {
    squares
        .iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .map(move |(c, col)| match (col, square) {
                    (Square::Start, Square::Start) => Some((r, c)),
                    (Square::End, Square::End) => Some((r, c)),
                    _ => None,
                })
        })
        .filter_map(|x| x)
        .last()
        .unwrap()
}

fn parse_map() -> Map {
    let squares: Vec<Vec<Square>> = read_to_vec("data/day12.txt", to_squares);
    let rows = squares.len();
    let cols = squares[0].len();
    let start = find_terminal(&squares, Square::Start);
    let end = find_terminal(&squares, Square::End);
    Map {
        squares,
        rows,
        cols,
        start,
        end,
    }
}

fn find_all_paths(map: &Map) -> HashMap<Cell, Cell> {
    let mut dist: HashMap<Cell, usize> = HashMap::new();
    let mut prev: HashMap<Cell, Cell> = HashMap::new();
    let mut vertices: HashSet<Cell> = HashSet::new();
    let mut min_heap = BinaryHeap::new();
    for r in 0..map.rows {
        for c in 0..map.cols {
            dist.insert((r, c), INFINITY);
            vertices.insert((r, c));
        }
    }

    dist.insert(map.end, 0);
    min_heap.push(State {
        position: map.end,
        cost: 0,
    });
    while let Some(state) = min_heap.pop() {
        let u = &state.position;
        vertices.remove(u);
        if dist[u] == INFINITY {
            break;
        }

        for v in map
            .neighbors(*u)
            .into_iter()
            .filter(|v| vertices.contains(v))
        {
            let alt = dist[u] + 1;
            if alt < dist[&v] {
                dist.insert(v, alt);
                prev.insert(v, *u);
                min_heap.push(State {
                    position: v,
                    cost: alt,
                })
            }
        }
    }

    prev
}

fn path_length(prev: &HashMap<Cell, Cell>, start: Cell, end: Cell) -> usize {
    let mut length = 0usize;
    if !prev.contains_key(&start) {
        return INFINITY;
    }
    let mut current = start;
    while current != end {
        length += 1;
        current = prev[&current];
    }
    length
}

fn part1(map: &Map, prev: &HashMap<Cell, Cell>) -> usize {
    path_length(prev, map.start, map.end)
}

fn part2(map: &Map, prev: &HashMap<Cell, Cell>) -> usize {
    (0..map.rows)
        .into_iter()
        .flat_map(|r| (0..map.cols).into_iter().map(move |c| (r, c)))
        .into_iter()
        .filter_map(|(r, c)| match map.squares[r][c] {
            Square::Ground(0) | Square::Start => Some(path_length(prev, (r, c), map.end)),
            _ => None,
        })
        .min()
        .unwrap()
}

pub fn run() {
    let map = parse_map();
    let prev = find_all_paths(&map);
    println!("== Day 12 ==");
    println!("Part 1: {}", part1(&map, &prev));
    println!("Part 2: {}", part2(&map, &prev))
}

use crate::utils::astar_search;
use crate::utils::read_to_vec;
use crate::utils::reconstruct_path;
use crate::utils::SearchInfo;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Instant;

#[derive(Eq, PartialEq)]
struct PathState {
    position: usize,
    cost: usize,
}

impl Ord for PathState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for PathState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Tunnels {
    valves: Vec<Valve>,
    start: usize,
    end: usize,
}

impl SearchInfo<usize, usize> for Tunnels {
    fn neighbors(&self, node: &usize) -> Vec<usize> {
        self.valves[*node].leads_to.clone()
    }

    fn heuristic(&self, _: &usize) -> usize {
        0
    }

    fn distance(&self, _: &usize, _: &usize) -> usize {
        1
    }

    fn start(&self) -> usize {
        self.start
    }

    fn is_goal(&self, node: &usize) -> bool {
        *node == self.end
    }

    fn infinity(&self) -> usize {
        usize::MAX
    }

    fn zero(&self) -> usize {
        0
    }
}

#[derive(Debug)]
struct Valve {
    flow_rate: usize,
    leads_to: Vec<usize>,
}

impl Clone for Valve {
    fn clone(&self) -> Self {
        Valve {
            flow_rate: self.flow_rate,
            leads_to: self.leads_to.clone(),
        }
    }
}

#[derive(Hash, Copy, Clone, Eq, PartialEq)]
struct Position {
    steps: usize,
    valve: usize,
}

#[derive(Hash, Copy, Clone, Eq, PartialEq)]
struct State {
    unopened: u64,
    pressure_released: usize,
    pos0: Position,
    pos1: Position,
}

fn to_valves(unopened: u64) -> Vec<usize> {
    (0..60)
        .into_iter()
        .filter(|s| (unopened & (1 << s)) > 0)
        .collect()
}

fn open(unopened: u64, v0: usize, v1: usize) -> u64 {
    let clear = ((1 << v0) | (1 << v1)) ^ u64::MAX;
    unopened & clear
}

fn can_move_to(unopened: u64, distances: &Vec<Vec<usize>>, pos: &Position) -> Vec<usize> {
    to_valves(unopened)
        .iter()
        .filter(|i| distances[pos.valve][**i] < pos.steps)
        .map(|i| *i)
        .collect()
}

impl State {
    fn max_outcome(&self, valves: &Vec<Valve>) -> usize {
        let mut flows = to_valves(self.unopened)
            .iter()
            .map(|i| valves[*i].flow_rate)
            .collect::<Vec<usize>>();
        flows.sort();
        flows.reverse();
        self.pressure_released
            + flows
                .iter()
                .enumerate()
                .map(|(i, f)| match 2 * i {
                    s if s < self.pos0.steps => (self.pos0.steps - s) * f,
                    _ => 0,
                })
                .sum::<usize>()
    }

    fn update(
        &self,
        valves: &Vec<Valve>,
        distances: &Vec<Vec<usize>>,
        move0: Option<usize>,
        move1: Option<usize>,
    ) -> State {
        let pos0 = match move0 {
            Some(valve) => Position {
                steps: self.pos0.steps - distances[self.pos0.valve][valve] - 1,
                valve,
            },
            None => Position { steps: 0, valve: 0 },
        };
        let pos1 = match move1 {
            Some(valve) => Position {
                steps: self.pos1.steps - distances[self.pos1.valve][valve] - 1,
                valve,
            },
            None => Position { steps: 0, valve: 0 },
        };

        State {
            unopened: open(self.unopened, pos0.valve, pos1.valve),
            pressure_released: self.pressure_released
                + pos0.steps * valves[pos0.valve].flow_rate
                + pos1.steps * valves[pos1.valve].flow_rate,
            pos0,
            pos1,
        }
    }

    fn get_moves(&self, distances: &Vec<Vec<usize>>) -> (Vec<usize>, Vec<usize>) {
        let mut moves0 = can_move_to(self.unopened, distances, &self.pos0);
        let mut moves1 = can_move_to(self.unopened, distances, &self.pos1);
        if moves0.len() == 1 && moves1.len() == 1 && moves0[0] == moves1[0] {
            let dist0 = distances[self.pos0.valve][moves0[0]];
            let dist1 = distances[self.pos1.valve][moves0[0]];
            if dist0 < dist1 {
                moves1.clear()
            } else {
                moves0.clear()
            }
        }

        (moves0, moves1)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.pressure_released
            .cmp(&other.pressure_released)
            .then_with(|| self.pos0.steps.cmp(&other.pos0.steps))
            .then_with(|| self.pos1.steps.cmp(&other.pos1.steps))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn to_valve(line: &str) -> (String, usize, Vec<String>) {
    let parts: Vec<&str> = line.split([' ', '=', ';', ',']).collect();
    let id = parts[1].to_string();
    let flow_rate = parts[5].parse::<usize>().unwrap();
    let leads_to = parts
        .into_iter()
        .rev()
        .filter(|s| s.len() > 0)
        .take_while(|s| !s.starts_with("valve"))
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    (id, flow_rate, leads_to)
}

fn read_valves() -> Vec<Valve> {
    let mut raw_valves: Vec<_> = read_to_vec("data/day16.txt", to_valve);
    raw_valves.sort();
    let lookup = raw_valves
        .iter()
        .enumerate()
        .map(|(i, v)| (v.0.clone(), i))
        .collect::<HashMap<String, usize>>();
    raw_valves
        .iter()
        .map(|v| Valve {
            flow_rate: v.1,
            leads_to: v.2.iter().map(|id| lookup[id]).collect(),
        })
        .collect()
}

fn distance_between(valves: &Vec<Valve>) -> Vec<Vec<usize>> {
    let mut distances = vec![vec![0; valves.len()]; valves.len()];
    let mut tunnels = Tunnels {
        valves: valves.clone(),
        start: 0,
        end: 0,
    };
    for i in 0..valves.len() {
        distances[i][i] = 0;
        tunnels.start = i;
        for j in i + 1..valves.len() {
            tunnels.end = j;
            let length = reconstruct_path(&astar_search(&tunnels).unwrap()).len() - 1;
            distances[i][j] = length;
            distances[j][i] = distances[i][j];
        }
    }

    distances
}

fn max_pressure_released(valves: &Vec<Valve>, use_elephant: bool) -> usize {
    let mut heap = BinaryHeap::new();
    let unopened = (0..valves.len())
        .enumerate()
        .filter_map(|(i, v)| {
            if valves[v].flow_rate > 0 {
                Some(i)
            } else {
                None
            }
        })
        .map(|i| 1u64 << i)
        .fold(0u64, |a, b| a | b);
    heap.push(State {
        unopened,
        pressure_released: 0,
        pos0: Position {
            steps: if use_elephant { 26 } else { 30 },
            valve: 0,
        },
        pos1: Position {
            steps: if use_elephant { 26 } else { 0 },
            valve: 0,
        },
    });

    let distances = distance_between(valves);

    let mut seen = HashSet::new();

    let mut most_pressure_released = 0;
    while let Some(current) = heap.pop() {
        seen.insert(current);
        if current.max_outcome(valves) < most_pressure_released {
            continue;
        } else if current.unopened == 0 {
            most_pressure_released = most_pressure_released.max(current.pressure_released)
        } else {
            let (moves0, moves1) = current.get_moves(&distances);

            if moves0.is_empty() && moves1.is_empty() {
                most_pressure_released = most_pressure_released.max(current.pressure_released);
            } else if moves1.is_empty() {
                for move0 in moves0 {
                    let next = current.update(valves, &distances, Some(move0), None);
                    if !seen.contains(&next) {
                        heap.push(next)
                    }
                }
            } else if moves0.is_empty() {
                for move1 in moves1 {
                    let next = current.update(valves, &distances, None, Some(move1));
                    if !seen.contains(&next) {
                        heap.push(next)
                    }
                }
            } else {
                for move0 in moves0 {
                    for move1 in &moves1 {
                        if move0 == *move1 {
                            continue;
                        }
                        let next = current.update(valves, &distances, Some(move0), Some(*move1));
                        if !seen.contains(&next) {
                            heap.push(next)
                        }
                    }
                }
            }
        }
    }
    most_pressure_released
}

fn part1(valves: &Vec<Valve>) -> usize {
    max_pressure_released(valves, false)
}

fn part2(valves: &Vec<Valve>) -> usize {
    max_pressure_released(valves, true)
}

pub fn run() {
    let valves = read_valves();
    println!("== Day 16 ==");
    println!("Part 1: {}", part1(&valves));
    let start = Instant::now();
    println!("Part 2: {}", part2(&valves));
    println!("{:?}", Instant::now() - start)
}

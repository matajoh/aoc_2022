use crate::utils::min_path;
use crate::utils::read_to_vec;
use crate::utils::GraphNode;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

struct Valve {
    flow_rate: usize,
    leads_to: Vec<usize>,
}

impl GraphNode for Valve {
    fn neighbors(&self) -> Vec<usize> {
        self.leads_to.clone()
    }
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

struct Cavern {
    valves: Vec<Valve>,
    distances: Vec<Vec<usize>>,
    use_elephant: bool,
}

#[derive(Hash, Copy, Clone, Eq, PartialEq)]
struct State {
    opened: u64,
    pressure_released: usize,
    pos0: Position,
    pos1: Position,
}

fn open_valves(opened: u64, valve0: usize, valve1: usize) -> u64 {
    opened | (1 << valve0) | (1 << valve1)
}

impl Cavern {
    fn start(&self) -> State {
        State {
            opened: 0,
            pressure_released: 0,
            pos0: Position {
                steps: if self.use_elephant { 26 } else { 30 },
                valve: 0,
            },
            pos1: Position {
                steps: if self.use_elephant { 26 } else { 0 },
                valve: 0,
            },
        }
    }

    fn new(valves: &Vec<Valve>, use_elephant: bool) -> Cavern {
        Cavern {
            valves: valves.clone(),
            distances: distance_between(valves),
            use_elephant: use_elephant,
        }
    }
    fn heuristic(&self, state: &State) -> usize {
        let mut flows = (0..self.valves.len())
            .map(|i| self.valves[i].flow_rate)
            .collect::<Vec<usize>>();
        flows.sort();
        flows.reverse();
        let steps = if self.use_elephant {
            state.pos0.steps.min(state.pos1.steps)
        } else {
            state.pos0.steps
        };

        state.pressure_released
            + flows
                .iter()
                .enumerate()
                .map(|(i, f)| match 2 * i {
                    s if s < steps => (steps - s) * f,
                    _ => 0,
                })
                .sum::<usize>()
    }

    fn update(&self, state: &State, move0: Option<usize>, move1: Option<usize>) -> State {
        let pos0 = match move0 {
            Some(valve) => Position {
                steps: state.pos0.steps - self.distances[state.pos0.valve][valve] - 1,
                valve,
            },
            None => Position { steps: 0, valve: 0 },
        };
        let pos1 = match move1 {
            Some(valve) => Position {
                steps: state.pos1.steps - self.distances[state.pos1.valve][valve] - 1,
                valve,
            },
            None => Position { steps: 0, valve: 0 },
        };

        State {
            opened: open_valves(state.opened, pos0.valve, pos1.valve),
            pressure_released: state.pressure_released
                + pos0.steps * self.valves[pos0.valve].flow_rate
                + pos1.steps * self.valves[pos1.valve].flow_rate,
            pos0,
            pos1,
        }
    }

    fn can_move_to(&self, opened: u64, pos: &Position) -> Vec<usize> {
        (0..self.valves.len())
            .filter(|i| opened & (1 << i) == 0)
            .filter(|i| self.distances[pos.valve][*i] < pos.steps)
            .filter(|i| self.valves[*i].flow_rate > 0)
            .collect()
    }

    fn get_moves(&self, state: &State) -> (Vec<usize>, Vec<usize>) {
        let mut moves0 = self.can_move_to(state.opened, &state.pos0);
        let mut moves1 = self.can_move_to(state.opened, &state.pos1);
        if moves0.len() == 1 && moves1.len() == 1 && moves0[0] == moves1[0] {
            let dist0 = self.distances[state.pos0.valve][moves0[0]];
            let dist1 = self.distances[state.pos1.valve][moves0[0]];
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
            .then_with(|| other.pos0.steps.cmp(&self.pos0.steps))
            .then_with(|| other.pos1.steps.cmp(&self.pos1.steps))
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
    for i in 0..valves.len() {
        distances[i][i] = 0;
        for j in i + 1..valves.len() {
            let length = min_path(valves, i, j).unwrap().len() - 1;
            distances[i][j] = length;
            distances[j][i] = distances[i][j];
        }
    }

    distances
}

fn max_pressure_released(valves: &Vec<Valve>, use_elephant: bool) -> usize {
    let cavern = Cavern::new(valves, use_elephant);
    let mut heap = BinaryHeap::new();
    heap.push(cavern.start());

    let mut seen = HashSet::new();

    let mut most_pressure_released = 0;
    while let Some(current) = heap.pop() {
        seen.insert(current);
        if cavern.heuristic(&current) < most_pressure_released {
            continue;
        } else {
            let (moves0, moves1) = cavern.get_moves(&current);

            if moves0.is_empty() && moves1.is_empty() {
                most_pressure_released = most_pressure_released.max(current.pressure_released);
            } else if moves1.is_empty() {
                for move0 in moves0 {
                    let next = cavern.update(&current, Some(move0), None);
                    if !seen.contains(&next) {
                        heap.push(next)
                    }
                }
            } else if moves0.is_empty() {
                for move1 in moves1 {
                    let next = cavern.update(&current, None, Some(move1));
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
                        let next = cavern.update(&current, Some(move0), Some(*move1));
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
    println!("Part 2: {}", part2(&valves));
}

use super::utils::read_to_vec;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct Valve {
    flow_rate: usize,
    leads_to: Vec<usize>,
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    unopened: HashSet<usize>,
    pressure_released: usize,
    me_steps: usize,
    me_valve: usize,
    elephant_steps: usize,
    elephant_valve: usize,
}

impl State {
    fn max_outcome(&self, valves: &Vec<Valve>) -> usize {
        let mut flows = self
            .unopened
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
                    s if s < self.me_steps => (self.me_steps - s) * f,
                    _ => 0,
                })
                .sum::<usize>()
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.pressure_released
            .cmp(&other.pressure_released)
            .then_with(|| self.me_steps.cmp(&other.me_steps))
            .then_with(|| self.elephant_steps.cmp(&other.elephant_steps))
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

fn min_path(valves: &Vec<Valve>, start: usize, end: usize) -> usize {
    let mut dist: HashMap<usize, usize> = HashMap::new();
    let mut prev: HashMap<usize, usize> = HashMap::new();
    let mut vertices: HashSet<usize> = HashSet::new();
    let mut min_heap = BinaryHeap::new();
    for i in 0..valves.len() {
        dist.insert(i, usize::MAX);
        vertices.insert(i);
    }

    dist.insert(start, 0);
    min_heap.push(PathState {
        position: start,
        cost: 0,
    });
    while let Some(state) = min_heap.pop() {
        let u = &state.position;
        vertices.remove(u);
        if dist[u] == usize::MAX {
            break;
        }

        for v in &valves[*u].leads_to {
            let alt = dist[u] + 1;
            if alt < dist[v] {
                dist.insert(*v, alt);
                prev.insert(*v, *u);
                min_heap.push(PathState {
                    position: *v,
                    cost: alt,
                })
            }
        }
    }

    let mut length = 0;
    let mut current = end;
    while current != start {
        current = prev[&current];
        length += 1;
    }
    length
}

fn distance_between(valves: &Vec<Valve>) -> Vec<Vec<usize>> {
    let mut distances = vec![vec![0; valves.len()]; valves.len()];
    for i in 0..valves.len() {
        distances[i][i] = 0;
        for j in i + 1..valves.len() {
            distances[i][j] = min_path(valves, i, j);
            distances[j][i] = distances[i][j];
        }
    }

    distances
}

fn can_move_to(
    unopened: &HashSet<usize>,
    pos: usize,
    steps: usize,
    distances: &Vec<Vec<usize>>,
) -> Vec<usize> {
    unopened
        .iter()
        .filter(|i| distances[pos][**i] < steps)
        .map(|i| *i)
        .collect()
}

fn max_pressure_released(valves: &Vec<Valve>, use_elephant: bool) -> usize {
    let mut heap = BinaryHeap::new();
    heap.push(State {
        unopened: (0..valves.len())
            .filter(|v| valves[*v].flow_rate > 0)
            .collect(),
        pressure_released: 0,
        me_steps: if use_elephant { 26 } else { 30 },
        me_valve: 0,
        elephant_steps: if use_elephant { 26 } else { 0 },
        elephant_valve: 0,
    });

    let distances = distance_between(valves);

    let mut most_pressure_released = 0;
    while let Some(current) = heap.pop() {
        if current.max_outcome(valves) < most_pressure_released {
            continue;
        } else if current.unopened.is_empty() {
            most_pressure_released = most_pressure_released.max(current.pressure_released)
        } else {
            let mut me_moves = can_move_to(
                &current.unopened,
                current.me_valve,
                current.me_steps,
                &distances,
            );

            let mut elephant_moves = can_move_to(
                &current.unopened,
                current.elephant_valve,
                current.elephant_steps,
                &distances,
            );

            if me_moves.len() == 1 && elephant_moves.len() == 1 && me_moves[0] == elephant_moves[0]
            {
                let me_dist = distances[current.me_valve][me_moves[0]];
                let elephant_dist = distances[current.elephant_valve][me_moves[0]];
                if me_dist < elephant_dist {
                    elephant_moves.clear()
                } else {
                    me_moves.clear()
                }
            }

            if me_moves.is_empty() && elephant_moves.is_empty() {
                most_pressure_released = most_pressure_released.max(current.pressure_released);
            } else if elephant_moves.is_empty() {
                for me_valve in me_moves {
                    let me_steps = current.me_steps - distances[current.me_valve][me_valve] - 1;
                    let pressure_released =
                        current.pressure_released + me_steps * valves[me_valve].flow_rate;
                    let unopened = current
                        .unopened
                        .difference(&HashSet::from([me_valve]))
                        .map(|v| *v)
                        .collect();
                    heap.push(State {
                        unopened,
                        me_steps,
                        pressure_released,
                        me_valve,
                        elephant_steps: current.elephant_steps,
                        elephant_valve: current.elephant_valve,
                    })
                }
            } else if me_moves.is_empty() {
                for elephant_valve in me_moves {
                    let elephant_steps = current.elephant_steps
                        - distances[current.elephant_valve][elephant_valve]
                        - 1;
                    let pressure_released = current.pressure_released
                        + elephant_steps * valves[elephant_valve].flow_rate;
                    let unopened = current
                        .unopened
                        .difference(&HashSet::from([elephant_valve]))
                        .map(|v| *v)
                        .collect();
                    heap.push(State {
                        unopened,
                        me_steps: current.me_steps,
                        pressure_released,
                        me_valve: current.me_valve,
                        elephant_steps,
                        elephant_valve,
                    })
                }
            } else {
                for me_valve in me_moves {
                    for elephant_valve in &elephant_moves {
                        if me_valve == *elephant_valve {
                            continue;
                        }

                        let me_steps = current.me_steps - distances[current.me_valve][me_valve] - 1;
                        let elephant_steps = current.elephant_steps
                            - distances[current.elephant_valve][*elephant_valve]
                            - 1;
                        let pressure_released = current.pressure_released
                            + elephant_steps * valves[*elephant_valve].flow_rate
                            + me_steps * valves[me_valve].flow_rate;
                        let unopened = current
                            .unopened
                            .difference(&HashSet::from([me_valve, *elephant_valve]))
                            .map(|v| *v)
                            .collect();
                        heap.push(State {
                            unopened,
                            me_steps,
                            pressure_released,
                            me_valve,
                            elephant_steps,
                            elephant_valve: *elephant_valve,
                        })
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
    println!("Part 2: {}", part2(&valves))
}

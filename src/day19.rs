use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::utils::read_to_vec;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Clone, Copy, Eq)]
struct Materials {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
}

impl PartialEq for Materials {
    fn eq(&self, other: &Self) -> bool {
        !(self.ore != other.ore
            || self.clay != other.clay
            || self.obsidian != other.obsidian
            || self.geode != other.geode)
    }
}

fn time_to_produce(quantity: i32, rate: i32) -> i32 {
    if quantity <= 0 {
        0
    } else if rate == 0 {
        i32::MAX
    } else if quantity % rate == 0 {
        quantity / rate
    } else {
        quantity / rate + 1
    }
}

impl Materials {
    fn contains(&self, m: &Materials) -> bool {
        m.ore <= self.ore
            && m.clay <= self.clay
            && m.obsidian <= self.obsidian
            && m.geode <= self.geode
    }

    fn can_build(&self, blueprint: &Blueprint) -> Vec<Robot> {
        [Robot::Ore, Robot::Clay, Robot::Obsidian, Robot::Geode]
            .iter()
            .filter_map(|r| {
                if self.contains(&blueprint[r]) {
                    Some(*r)
                } else {
                    None
                }
            })
            .collect()
    }

    fn is_zero(&self) -> bool {
        !(self.ore > 0 || self.clay > 0 || self.obsidian > 0 || self.geode > 0)
    }

    fn subtract(&self, m: &Materials) -> Materials {
        Materials {
            ore: self.ore - m.ore,
            clay: self.clay - m.clay,
            obsidian: self.obsidian - m.obsidian,
            geode: self.geode - m.geode,
        }
    }

    fn add(&self, m: &Materials) -> Materials {
        Materials {
            ore: self.ore + m.ore,
            clay: self.clay + m.clay,
            obsidian: self.obsidian + m.obsidian,
            geode: self.geode + m.geode,
        }
    }

    fn multiply(&self, m: i32) -> Materials {
        Materials {
            ore: self.ore * m,
            clay: self.clay * m,
            obsidian: self.obsidian * m,
            geode: self.geode * m,
        }
    }

    fn add_robot(&self, r: Robot) -> Materials {
        let mut mats = *self;
        match r {
            Robot::Ore => mats.ore += 1,
            Robot::Clay => mats.clay += 1,
            Robot::Obsidian => mats.obsidian += 1,
            Robot::Geode => mats.geode += 1,
        }
        mats
    }

    fn time_to_produce(&self, robots: &Materials) -> Materials {
        Materials {
            ore: time_to_produce(self.ore, robots.ore),
            clay: time_to_produce(self.clay, robots.clay),
            obsidian: time_to_produce(self.obsidian, robots.obsidian),
            geode: time_to_produce(self.geode, robots.geode),
        }
    }

    fn max(&self) -> i32 {
        self.ore.max(self.clay.max(self.obsidian.max(self.geode)))
    }

    fn none() -> Materials {
        Materials {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }

    fn score(&self) -> i32 {
        self.ore + self.clay * 10 + self.obsidian * 100 + self.geode * 1000
    }
}

type Blueprint = HashMap<Robot, Materials>;

fn to_blueprints(line: &str) -> Blueprint {
    let parts = line.split(['.', ':', ' ']).collect::<Vec<&str>>();
    let mut blueprint = HashMap::new();
    blueprint.insert(
        Robot::Ore,
        Materials {
            ore: parts[7].parse().unwrap(),
            clay: 0,
            obsidian: 0,
            geode: 0,
        },
    );
    blueprint.insert(
        Robot::Clay,
        Materials {
            ore: parts[14].parse().unwrap(),
            clay: 0,
            obsidian: 0,
            geode: 0,
        },
    );
    blueprint.insert(
        Robot::Obsidian,
        Materials {
            ore: parts[21].parse().unwrap(),
            clay: parts[24].parse().unwrap(),
            obsidian: 0,
            geode: 0,
        },
    );
    blueprint.insert(
        Robot::Geode,
        Materials {
            ore: parts[31].parse().unwrap(),
            clay: 0,
            obsidian: parts[34].parse().unwrap(),
            geode: 0,
        },
    );
    blueprint
}

#[derive(Debug, Clone)]
struct State {
    minute: usize,
    materials: Materials,
    robots: Materials,
    actions: Vec<(usize, Robot)>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.materials
            .score()
            .cmp(&other.materials.score())
            .then_with(|| self.robots.score().cmp(&other.robots.score()))
            .then_with(|| other.minute.cmp(&self.minute))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for State {}

impl State {
    fn init() -> State {
        State {
            minute: 0,
            materials: Materials::none(),
            robots: Materials::none().add_robot(Robot::Ore),
            actions: vec![],
        }
    }

    fn mine(&self, minutes: usize) -> State {
        State {
            minute: self.minute + minutes,
            materials: self.materials.add(&self.robots.multiply(minutes as i32)),
            robots: self.robots,
            actions: self.actions.clone(),
        }
    }

    fn build(&self, blueprint: &Blueprint, robot: Robot) -> State {
        let mut actions = self.actions.clone();
        actions.push((self.minute, robot));
        State {
            minute: self.minute + 1,
            materials: self
                .materials
                .add(&self.robots)
                .subtract(&blueprint[&robot]),
            robots: self.robots.add_robot(robot),
            actions,
        }
    }

    fn replay(&self, blueprint: &Blueprint, minutes: usize) {
        let mut i = 0;
        let mut state = State::init();
        while i < self.actions.len() {
            println!("== Minute {} ==", state.minute + 1);
            let build = if state.minute == self.actions[i].0 {
                let robot = self.actions[i].1;
                println!("Spend {:?} to build {:?}", blueprint[&robot], robot);
                i += 1;
                Some(robot)
            } else {
                None
            };
            println!("robots: {:?}", state.robots);

            state = match build {
                Some(robot) => state.build(blueprint, robot),
                _ => state.mine(1),
            };
            println!("mats: {:?}", state.materials);
            println!();
        }

        while state.minute < minutes {
            println!("== Minute {} ==", state.minute + 1);
            println!("robots: {:?}", state.robots);
            state = state.mine(1);
            println!("mats: {:?}", state.materials);
            println!();
        }
    }
}

fn build(blueprint: &Blueprint, robot: Robot, state: &State, limit: usize) -> Vec<State> {
    let needed = blueprint[&robot];
    let mut remaining = needed.subtract(&state.materials);
    if robot == Robot::Ore {
        let time = time_to_produce(remaining.ore, state.robots.ore);
        return vec![state.mine(time as usize).build(blueprint, robot)];
    }

    let mut results = vec![];
    let mut plans = vec![state.clone()];
    while let Some(current) = plans.pop() {
        if current.minute >= limit {
            continue;
        }

        remaining = needed.subtract(&current.materials);
        let time = remaining.time_to_produce(&current.robots);
        if time.max() < i32::MAX {
            let build = current.mine(time.max() as usize).build(blueprint, robot);
            results.push(build);
        }

        if time.ore > 0 {
            plans.extend(build(blueprint, Robot::Ore, &current, limit))
        }

        if time.clay > 0 {
            plans.extend(build(blueprint, Robot::Clay, &current, limit))
        }

        if time.obsidian > 0 {
            plans.extend(build(blueprint, Robot::Obsidian, &current, limit))
        }
    }

    results
}

fn most_geodes(blueprint: &Blueprint, minutes: usize) -> usize {
    let mut heap = BinaryHeap::new();
    heap.push(State::init());
    let mut best = State::init();
    while let Some(current) = heap.pop() {
        if current.minute > minutes {
            continue;
        }

        let end = current.mine(minutes - current.minute);
        if end > best {
            best = end
        }

        for next in build(blueprint, Robot::Geode, &current, minutes) {
            heap.push(next)
        }
    }
    best.replay(blueprint, minutes);
    println!("{}", best.materials.geode);
    best.materials.geode as usize
}

fn part1(blueprints: &Vec<Blueprint>) -> usize {
    blueprints
        .iter()
        .enumerate()
        .map(|(i, b)| (i + 1) * most_geodes(b, 24))
        .sum()
}

fn part2(blueprints: &Vec<Blueprint>) -> usize {
    blueprints
        .iter()
        .take(3)
        .map(|b| most_geodes(b, 32))
        .reduce(|a, b| a * b)
        .unwrap()
}

pub fn run() {
    let blueprints = read_to_vec("data/day19.txt", to_blueprints);
    println!("== Day 19 == ");
    println!("Part 1: {}", part1(&blueprints));
    println!("Part 2: {}", part2(&blueprints))
}

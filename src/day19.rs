use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::utils::read_to_vec;

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Clone, Copy, Eq, Hash)]
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

#[derive(Copy, Clone, Hash)]
struct State {
    minute: usize,
    materials: Materials,
    robots: Materials,
    potential: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score()
            .total_cmp(&other.score())
            .then_with(|| other.minute.cmp(&self.minute))
            .then_with(|| self.potential.cmp(&other.potential))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.materials.eq(&other.materials) && self.robots.eq(&other.robots)
    }
}

impl Eq for State {}

impl State {
    fn init() -> State {
        State {
            minute: 0,
            materials: Materials::none(),
            robots: Materials::none().add_robot(Robot::Ore),
            potential: 0,
        }
    }

    fn score(&self) -> f32 {
        if self.materials.geode > 0 {
            (self.materials.geode as f32) / (self.minute as f32)
        } else {
            0.0
        }
    }

    fn mine(&self, duration: i32) -> State {
        if duration == 0 {
            return *self;
        }

        State {
            minute: self.minute + duration as usize,
            materials: self.materials.add(&self.robots.multiply(duration)),
            robots: self.robots,
            potential: 0,
        }
    }

    fn time_to_build(&self, cost: &Materials) -> Materials {
        let remaining = cost.subtract(&self.materials);
        remaining.time_to_produce(&self.robots)
    }

    fn estimate_geodes(&self, blueprint: &Blueprint, minutes: usize) -> State {
        let obsidian = blueprint[&Robot::Geode].obsidian;
        let clay = blueprint[&Robot::Obsidian].clay;
        let mut materials = self.materials;
        let mut robots = self.robots;
        for _ in self.minute..minutes {
            if materials.obsidian >= obsidian {
                robots.geode += 1;
                materials.obsidian -= obsidian;
            } else if materials.clay >= clay && robots.obsidian < obsidian {
                robots.obsidian += 1;
                materials.clay -= clay;
            } else if robots.clay < clay {
                robots.clay += 1;
            }
            materials = materials.add(&robots);
        }

        State {
            minute: self.minute,
            materials: self.materials,
            robots: self.robots,
            potential: materials.geode,
        }
    }

    fn build(&self, blueprint: &Blueprint, robot: Robot) -> State {
        State {
            minute: self.minute + 1,
            materials: self
                .materials
                .add(&self.robots)
                .subtract(&blueprint[&robot]),
            robots: self.robots.add_robot(robot),
            potential: 0,
        }
    }
}

fn most_geodes(blueprint: &Blueprint, minutes: usize) -> usize {
    let mut seen = HashSet::new();
    let mut heap = BinaryHeap::new();
    heap.push(State::init());
    let mut best = State::init();
    let ore = &blueprint[&Robot::Ore];
    let clay = &blueprint[&Robot::Clay];
    let obsidian = &blueprint[&Robot::Obsidian];
    let geode = &blueprint[&Robot::Geode];
    let max_ore = ore.ore.max(clay.ore).max(obsidian.ore).max(geode.ore);
    let max_clay = obsidian.clay;
    let max_obsidian = geode.obsidian;
    while let Some(current) = heap.pop() {
        if current.minute > minutes || current.potential < best.materials.geode {
            continue;
        }

        if current.materials.geode > best.materials.geode {
            best = current
        }

        let time = current.time_to_build(geode).max();
        if time < i32::MAX {
            let next = current
                .mine(time)
                .build(blueprint, Robot::Geode)
                .estimate_geodes(blueprint, minutes);
            if seen.insert(next) {
                heap.push(next);
            }
        }
        if current.robots.ore < max_ore {
            let time = current.time_to_build(ore).max();
            let next = current
                .mine(time)
                .build(blueprint, Robot::Ore)
                .estimate_geodes(blueprint, minutes);
            if seen.insert(next) {
                heap.push(next)
            }
        }
        if current.robots.clay < max_clay {
            let time = current.time_to_build(clay).max();
            let next = current
                .mine(time)
                .build(blueprint, Robot::Clay)
                .estimate_geodes(blueprint, minutes);
            if seen.insert(next) {
                heap.push(next);
            }
        }
        if current.robots.obsidian < max_obsidian {
            let time = current.time_to_build(obsidian).max();
            if time < i32::MAX {
                let next = current
                    .mine(time)
                    .build(blueprint, Robot::Obsidian)
                    .estimate_geodes(blueprint, minutes);
                if seen.insert(next) {
                    heap.push(next)
                }
            }
        }
    }

    best = best.mine((minutes - best.minute) as i32);
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

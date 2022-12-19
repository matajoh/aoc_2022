use crate::utils::read_to_vec;

enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug)]
struct Materials {
    ore: usize,
    clay: usize,
    obsidian: usize,
}

impl Materials {
    fn contains(&self, m: &Materials) -> bool {
        m.ore <= self.ore && m.clay <= self.clay && m.obsidian <= self.obsidian
    }

    fn can_build(&self, blueprint: Blueprint) -> Vec<Robot> {
        [
            (Robot::Ore, blueprint.ore),
            (Robot::Clay, blueprint.clay),
            (Robot::Obsidian, blueprint.obsidian),
            (Robot::Geode, blueprint.geode),
        ]
        .iter()
        .filter_map(|(r, m)| if self.contains(m) { Some(*r) } else { None })
        .collect()
    }
}

#[derive(Debug)]
struct Blueprint {
    ore: Materials,
    clay: Materials,
    obsidian: Materials,
    geode: Materials,
}

fn to_blueprints(line: &str) -> Blueprint {
    let parts = line.split(['.', ':', ' ']).collect::<Vec<&str>>();
    Blueprint {
        ore: Materials {
            ore: parts[7].parse().unwrap(),
            clay: 0,
            obsidian: 0,
        },
        clay: Materials {
            ore: parts[14].parse().unwrap(),
            clay: 0,
            obsidian: 0,
        },
        obsidian: Materials {
            ore: parts[21].parse().unwrap(),
            clay: parts[24].parse().unwrap(),
            obsidian: 0,
        },
        geode: Materials {
            ore: parts[31].parse().unwrap(),
            clay: 0,
            obsidian: parts[34].parse().unwrap(),
        },
    }
}

fn part1(blueprints: &Vec<Blueprint>) -> usize {}

pub fn run() {
    let blueprints = read_to_vec("data/day19.txt", to_blueprints);
    println!("== Day 19 == ");
    println!("Blueprints: {:?}", &blueprints)
}

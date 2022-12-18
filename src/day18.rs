use std::collections::HashSet;

use crate::utils::read_to_vec;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Cube(i32, i32, i32);

impl Cube {
    fn neighbors(&self) -> [Cube; 6] {
        [
            Cube(self.0 - 1, self.1, self.2),
            Cube(self.0 + 1, self.1, self.2),
            Cube(self.0, self.1 - 1, self.2),
            Cube(self.0, self.1 + 1, self.2),
            Cube(self.0, self.1, self.2 - 1),
            Cube(self.0, self.1, self.2 + 1),
        ]
    }
}

struct Bounds {
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
    front: i32,
    back: i32,
}

impl Bounds {
    fn contains(&self, cube: &Cube) -> bool {
        !(cube.0 <= self.left
            || cube.0 >= self.right
            || cube.1 <= self.top
            || cube.1 >= self.bottom
            || cube.2 <= self.front
            || cube.2 >= self.back)
    }

    fn from(cubes: &HashSet<Cube>) -> Bounds {
        let left = cubes.iter().map(|c| c.0).min().unwrap() - 2;
        let right = cubes.iter().map(|c| c.0).max().unwrap() + 2;
        let top = cubes.iter().map(|c| c.1).min().unwrap() - 2;
        let bottom = cubes.iter().map(|c| c.1).max().unwrap() + 2;
        let front = cubes.iter().map(|c| c.2).min().unwrap() - 2;
        let back = cubes.iter().map(|c| c.2).max().unwrap() + 2;
        Bounds {
            left,
            right,
            top,
            bottom,
            front,
            back,
        }
    }
}

fn to_cube(line: &str) -> Cube {
    let parts = line
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i32>>();
    Cube(parts[0], parts[1], parts[2])
}

fn part1(cubes: &HashSet<Cube>) -> usize {
    cubes
        .iter()
        .map(|c| 6 - c.neighbors().iter().filter(|n| cubes.contains(n)).count())
        .sum()
}

fn part2(cubes: &HashSet<Cube>) -> usize {
    let mut external = HashSet::new();
    let bounds = Bounds::from(&cubes);
    let mut heap = vec![Cube(bounds.left + 1, bounds.top + 1, bounds.front + 1)];
    while let Some(current) = heap.pop() {
        external.insert(current);
        for n in current.neighbors() {
            if bounds.contains(&n) && !cubes.contains(&n) && !external.contains(&n) {
                heap.push(n)
            }
        }
    }

    cubes
        .iter()
        .map(|c| {
            6 - c
                .neighbors()
                .iter()
                .filter(|n| !external.contains(n))
                .count()
        })
        .sum()
}

pub fn run() {
    let cubes = read_to_vec("data/day18.txt", to_cube).into_iter().collect();
    println!("== Day 18 ==");
    println!("Part 1: {}", part1(&cubes));
    println!("Part 2: {}", part2(&cubes))
}

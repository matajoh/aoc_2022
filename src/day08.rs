use super::utils::read_to_vec;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_trees() -> Vec<Vec<i32>> {
    let lines: Vec<String> = read_to_vec("data/day08.txt", |a| a.to_string());
    lines
        .iter()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect()
}

fn visibility_pass(visibility: &mut Vec<Vec<bool>>, trees: &Vec<Vec<i32>>, direction: Direction) {
    let (rows, cols) = (trees.len(), trees[0].len());

    let mut highest = vec![-1; cols];
    match direction {
        Direction::Left | Direction::Right => {
            for c in 0..cols {
                for r in 0..rows {
                    let col = if let Direction::Left = direction {
                        cols - c - 1
                    } else {
                        c
                    };
                    let t = trees[r][col];
                    let h = highest[r];
                    visibility[r][col] = visibility[r][col] || t > h;
                    highest[r] = if t > h { t } else { h }
                }
            }
        }
        Direction::Up | Direction::Down => {
            for r in 0..rows {
                for c in 0..cols {
                    let row = if let Direction::Up = direction {
                        rows - r - 1
                    } else {
                        r
                    };
                    let t = trees[row][c];
                    let h = highest[c];
                    visibility[row][c] = visibility[row][c] || t > h;
                    highest[c] = if t > h { t } else { h }
                }
            }
        }
    }
}

fn part1(trees: &Vec<Vec<i32>>) -> usize {
    let (rows, cols) = (trees.len(), trees[0].len());
    let mut visibility = vec![vec![false; cols]; rows];

    for dir in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ] {
        visibility_pass(&mut visibility, trees, dir)
    }
    return visibility
        .iter()
        .map(|row| row.iter().filter(|v| **v).count())
        .sum();
}

#[derive(Clone)]
struct Scenery {
    up: i32,
    down: i32,
    left: i32,
    right: i32,
}

impl Scenery {
    fn score(&self) -> i32 {
        self.up * self.down * self.left * self.right
    }
}

fn scenery_pass(scenery: &mut Vec<Vec<Scenery>>, trees: &Vec<Vec<i32>>, direction: Direction) {
    let (rows, cols) = (trees.len(), trees[0].len());

    match direction {
        Direction::Left => {
            let mut index: Vec<Vec<Option<usize>>> = vec![vec![None; rows]; 10];
            for c in 0..cols {
                for r in 0..rows {
                    let t = trees[r][c] as usize;
                    let last = index[t..].iter().filter_map(|i| i[r].as_ref()).max();
                    scenery[r][c].left = match last {
                        None => c as i32,
                        Some(last) => (c - last).try_into().unwrap(),
                    };
                    index[t][r] = Some(c)
                }
            }
        }
        Direction::Right => {
            let mut index: Vec<Vec<Option<usize>>> = vec![vec![None; rows]; 10];
            for c in (0..cols).rev() {
                for r in 0..rows {
                    let t = trees[r][c] as usize;
                    let last = index[t..].iter().filter_map(|i| i[r].as_ref()).min();
                    scenery[r][c].right = match last {
                        None => (cols - c - 1).try_into().unwrap(),
                        Some(last) => (last - c).try_into().unwrap(),
                    };
                    index[t][r] = Some(c)
                }
            }
        }
        Direction::Down => {
            let mut index: Vec<Vec<Option<usize>>> = vec![vec![None; cols]; 10];
            for r in 0..rows {
                for c in 0..cols {
                    let t = trees[r][c] as usize;
                    let last = index[t..].iter().filter_map(|i| i[c].as_ref()).max();
                    scenery[r][c].down = match last {
                        None => r as i32,
                        Some(last) => (r - last).try_into().unwrap(),
                    };
                    index[t][c] = Some(r)
                }
            }
        }
        Direction::Up => {
            let mut index: Vec<Vec<Option<usize>>> = vec![vec![None; cols]; 10];
            for r in (0..rows).rev() {
                for c in 0..cols {
                    let t = trees[r][c] as usize;
                    let last = index[t..].iter().filter_map(|i| i[c].as_ref()).min();
                    scenery[r][c].up = match last {
                        None => (rows - r - 1).try_into().unwrap(),
                        Some(last) => (last - r).try_into().unwrap(),
                    };
                    index[t][c] = Some(r)
                }
            }
        }
    }
}

fn part2(trees: &Vec<Vec<i32>>) -> i32 {
    let (rows, cols) = (trees.len(), trees[0].len());
    let mut scenery = vec![
        vec![
            Scenery {
                up: 0,
                down: 0,
                left: 0,
                right: 0
            };
            cols
        ];
        rows
    ];

    for dir in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ] {
        scenery_pass(&mut scenery, trees, dir)
    }

    scenery
        .iter()
        .map(|r| r.iter().map(|i| i.score()).max().unwrap())
        .max()
        .unwrap()
}

pub fn run() {
    let trees = parse_trees();
    println!("== Day 08 ==");
    println!("Part 1: {}", part1(&trees));
    println!("Part 2: {}", part2(&trees))
}

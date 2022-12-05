use super::utils::read_to_vec;

struct Move {
    count: usize,
    from: usize,
    to: usize,
}

fn to_crates(line: &str) -> Vec<(usize, char)> {
    line.char_indices()
        .map(|(i, c)| match (i, c) {
            (_, '[') => None,
            (_, ']') => None,
            (_, ' ') => None,
            (_, c) => Some((i / 4, c)),
        })
        .filter_map(|a| a)
        .collect()
}

fn to_move(line: &str) -> Move {
    let parts: Vec<&str> = line.split(" ").collect();
    let count: usize = parts[1].trim().parse().unwrap();
    let from: usize = parts[3].trim().parse().unwrap();
    let to: usize = parts[5].trim().parse().unwrap();
    Move { count, from, to }
}

fn parse() -> (Vec<String>, Vec<Move>) {
    let mut stacks: Vec<String> = [0; 9].iter().map(|_| "".to_string()).collect();
    let mut moves: Vec<Move> = vec![];
    let lines = read_to_vec("data/day05.txt", |line| line.to_string());
    for line in lines {
        if line.starts_with(" 1") || line.len() == 0 {
            continue;
        }

        if line.starts_with("move") {
            moves.push(to_move(&line))
        } else {
            let crates = to_crates(&line);
            for (i, c) in crates {
                stacks[i].insert(0, c)
            }
        }
    }

    return (stacks, moves);
}

fn message(stacks: &Vec<String>) -> String {
    stacks
        .iter()
        .map(|s| match s.chars().last() {
            Some(c) => c,
            None => ' ',
        })
        .collect()
}

fn part1() -> String {
    let (mut stacks, moves) = parse();

    for m in moves {
        for _ in 0..m.count {
            if let Some(c) = stacks[m.from - 1].pop() {
                stacks[m.to - 1].push(c)
            }
        }
    }

    message(&stacks)
}

fn part2() -> String {
    let (mut stacks, moves) = parse();

    for m in moves {
        let end = stacks[m.from - 1].len();
        let start = end - m.count;
        let crates: String = stacks[m.from - 1].drain(start..end).collect();
        stacks[m.to - 1] += crates.as_str()
    }

    message(&stacks)
}

pub fn run() {
    println!("== Day 05 ==");
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2())
}

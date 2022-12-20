use crate::utils::read_some_to_vec;

enum Move {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

struct Round {
    first: Move,
    second: Move,
}

impl Round {
    fn outcome(&self) -> Outcome {
        match (&self.first, &self.second) {
            (Move::Rock, Move::Rock) => Outcome::Draw,
            (Move::Rock, Move::Paper) => Outcome::Win,
            (Move::Rock, Move::Scissors) => Outcome::Lose,
            (Move::Paper, Move::Rock) => Outcome::Lose,
            (Move::Paper, Move::Paper) => Outcome::Draw,
            (Move::Paper, Move::Scissors) => Outcome::Win,
            (Move::Scissors, Move::Rock) => Outcome::Win,
            (Move::Scissors, Move::Paper) => Outcome::Lose,
            (Move::Scissors, Move::Scissors) => Outcome::Draw,
        }
    }

    fn score(&self) -> i32 {
        let outcome_score = match self.outcome() {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Draw => 3,
        };
        let move_score = match &self.second {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        };
        outcome_score + move_score
    }
}

fn to_round_part1(line: &str) -> Option<Round> {
    let parts: Vec<&str> = line.split(" ").collect();
    let lhs = match parts[0] {
        "A" => Some(Move::Rock),
        "B" => Some(Move::Paper),
        "C" => Some(Move::Scissors),
        _ => None,
    };
    let rhs = match parts[1] {
        "X" => Some(Move::Rock),
        "Y" => Some(Move::Paper),
        "Z" => Some(Move::Scissors),
        _ => None,
    };

    match (lhs, rhs) {
        (Some(first), Some(second)) => Some(Round { first, second }),
        _ => None,
    }
}

fn to_round_part2(line: &str) -> Option<Round> {
    let parts: Vec<&str> = line.split(" ").collect();
    let round = match (parts[0], parts[1]) {
        ("A", "X") => Some((Move::Rock, Move::Scissors)),
        ("A", "Y") => Some((Move::Rock, Move::Rock)),
        ("A", "Z") => Some((Move::Rock, Move::Paper)),
        ("B", "X") => Some((Move::Paper, Move::Rock)),
        ("B", "Y") => Some((Move::Paper, Move::Paper)),
        ("B", "Z") => Some((Move::Paper, Move::Scissors)),
        ("C", "X") => Some((Move::Scissors, Move::Paper)),
        ("C", "Y") => Some((Move::Scissors, Move::Scissors)),
        ("C", "Z") => Some((Move::Scissors, Move::Rock)),
        _ => None,
    };

    match round {
        Some((first, second)) => Some(Round { first, second }),
        _ => None,
    }
}

fn total_score(rounds: Vec<Round>) -> i32 {
    rounds.iter().map(|r| -> i32 { return r.score() }).sum()
}

pub fn run() {
    let part1_rounds: Vec<Round> = read_some_to_vec("./data/day02.txt", to_round_part1);
    let part2_rounds: Vec<Round> = read_some_to_vec("./data/day02.txt", to_round_part2);

    println!("== Day 02 ==");
    println!("Part 1: {}", total_score(part1_rounds));
    println!("Part 2: {}", total_score(part2_rounds));
}

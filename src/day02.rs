use super::utils::read_and_convert;
use std::fmt;

enum Move {
    Rock,
    Paper,
    Scissors,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Move::Rock => write!(f, "Rock"),
            Move::Paper => write!(f, "Paper"),
            Move::Scissors => write!(f, "Scissors"),
        }
    }
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

impl fmt::Display for Round {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Round({} {})", self.first, self.second)
    }
}

impl Round {
    fn outcome(&self) -> Outcome {
        return match (&self.first, &self.second) {
            (Move::Rock, Move::Rock) => Outcome::Draw,
            (Move::Rock, Move::Paper) => Outcome::Win,
            (Move::Rock, Move::Scissors) => Outcome::Lose,
            (Move::Paper, Move::Rock) => Outcome::Lose,
            (Move::Paper, Move::Paper) => Outcome::Draw,
            (Move::Paper, Move::Scissors) => Outcome::Win,
            (Move::Scissors, Move::Rock) => Outcome::Win,
            (Move::Scissors, Move::Paper) => Outcome::Lose,
            (Move::Scissors, Move::Scissors) => Outcome::Draw,
        };
    }

    fn score(&self) -> i32 {
        let outcome_score = match self.outcome() {
            Outcome::Win => 6i32,
            Outcome::Lose => 0i32,
            Outcome::Draw => 3i32,
        };
        let move_score = match &self.second {
            Move::Rock => 1i32,
            Move::Paper => 2i32,
            Move::Scissors => 3i32,
        };
        return outcome_score + move_score;
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

    return match (lhs, rhs) {
        (Some(first), Some(second)) => Some(Round { first, second }),
        _ => None,
    };
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

    return match round {
        Some((first, second)) => Some(Round { first, second }),
        _ => None,
    };
}

fn total_score(rounds: Vec<Round>) -> i32 {
    return rounds.iter().map(|r| -> i32 { return r.score() }).sum();
}

pub fn run() {
    let part1_rounds: Vec<Round> = read_and_convert("./data/day02.txt", to_round_part1);
    let part2_rounds: Vec<Round> = read_and_convert("./data/day02.txt", to_round_part2);

    println!("== Day 02 ==");
    println!("Part 1: {}", total_score(part1_rounds));
    println!("Part 2: {}", total_score(part2_rounds));
}

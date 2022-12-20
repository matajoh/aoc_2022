use crate::utils::read_some_to_vec;
use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
enum Packet {
    Value(usize),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        compare(self, other)
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(compare(self, other))
    }
}

fn compare(left: &Packet, right: &Packet) -> Ordering {
    match [left, right] {
        [Packet::Value(lhs), Packet::Value(rhs)] => lhs.cmp(rhs),
        [Packet::List(lhs), Packet::List(rhs)] => {
            let count = if lhs.len() < rhs.len() {
                lhs.len()
            } else {
                rhs.len()
            };
            for i in 0..count {
                match compare(&lhs[i], &rhs[i]) {
                    Ordering::Less => return Ordering::Less,
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Equal => {}
                }
            }
            if lhs.len() < rhs.len() {
                Ordering::Less
            } else if rhs.len() < lhs.len() {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }
        [Packet::List(lhs), Packet::Value(_)] => {
            if lhs.is_empty() {
                Ordering::Less
            } else {
                match compare(&lhs[0], right) {
                    Ordering::Less => Ordering::Less,
                    _ => Ordering::Greater,
                }
            }
        }
        [Packet::Value(_), Packet::List(rhs)] => {
            if rhs.is_empty() {
                Ordering::Greater
            } else {
                match compare(left, &rhs[0]) {
                    Ordering::Greater => Ordering::Greater,
                    _ => Ordering::Less,
                }
            }
        }
    }
}

fn next_token(input: &str) -> (Option<&str>, &str) {
    match input.chars().next() {
        Some('[') | Some(']') => (Some(&input[..1]), &input[1..]),
        Some(',') => next_token(&input[1..]),
        Some(c) if c.is_ascii_digit() => match input.find(|c: char| c == ',' || c == ']') {
            Some(i) => (Some(&input[..i]), &input[i..]),
            None => (Some(&input[..]), ""),
        },
        Some(_) => panic!("invalid char"),
        None => (None, ""),
    }
}

fn parse_packet(input: &str) -> (Packet, &str) {
    let mut tokens = input;
    let mut stack: Vec<Vec<Packet>> = vec![];
    let mut current: Vec<Packet> = vec![];
    while let (Some(token), remainder) = next_token(tokens) {
        tokens = remainder;
        match token {
            "[" => {
                stack.push(current);
                current = vec![];
            }
            "]" => {
                let packet = Packet::List(current);
                current = stack.pop().unwrap();
                current.push(packet)
            }
            value => current.push(Packet::Value(value.parse().unwrap())),
        }
    }
    (Packet::List(current), tokens)
}

fn to_packet(line: &str) -> Option<Packet> {
    match line.trim() {
        "" => None,
        input => Some(parse_packet(input).0),
    }
}

fn read_packets() -> Vec<Packet> {
    read_some_to_vec("data/day13.txt", to_packet)
}

fn part1(packets: &Vec<Packet>) -> usize {
    packets
        .chunks(2)
        .enumerate()
        .map(|(i, chunk)| match compare(&chunk[0], &chunk[1]) {
            Ordering::Greater => 0,
            _ => i + 1,
        })
        .sum()
}

fn part2(packets: &Vec<Packet>) -> usize {
    let dividers = vec![parse_packet("[[[2]]").0, parse_packet("[[6]]").0];
    let mut all_packets = vec![];
    all_packets.extend(packets);
    all_packets.extend(&dividers);
    all_packets.sort();
    all_packets
        .into_iter()
        .enumerate()
        .map(|(i, p)| match p {
            p if p == &dividers[0] || p == &dividers[1] => i + 1,
            _ => 1,
        })
        .reduce(|a, b| a * b)
        .unwrap()
}

pub fn run() {
    let packets = read_packets();
    println!("== Day 13 ==");
    println!("Part 1: {}", part1(&packets));
    println!("Part 2: {}", part2(&packets))
}

use super::utils::read_and_convert;
use std::collections::HashSet;

fn to_priority(item: char) -> i32 {
    const LOWER: i32 = 'a' as i32;
    const UPPER: i32 = 'A' as i32;
    let priority: i32 = item as i32;
    if item.is_lowercase() {
        return priority - LOWER + 1;
    }

    return priority - UPPER + 27;
}

fn to_item_set(rucksack: &str) -> HashSet<char> {
    return rucksack.chars().into_iter().collect();
}

fn common_items(lhs: String, rhs: &str) -> String {
    return match (lhs.as_str(), rhs) {
        ("", value) => value.to_string(),
        (a, b) => {
            let first = to_item_set(a);
            let second = to_item_set(b);
            return first.intersection(&second).collect();
        }
    };
}

fn common_item(rucksacks: &[String]) -> char {
    let common = rucksacks
        .iter()
        .fold("".to_string(), |acc, r| common_items(acc, r));
    assert!(common.len() == 1);
    return common.chars().next().unwrap();
}

fn compartments(rucksack: &String) -> (String, String) {
    let length = rucksack.len();
    let first = rucksack[0..length / 2].to_string();
    let second = rucksack[length / 2..length].to_string();
    return (first, second);
}

fn part1(rucksacks: &Vec<String>) -> i32 {
    return rucksacks
        .iter()
        .map(|r| compartments(r))
        .map(|(first, second)| common_item(&[first, second]))
        .map(|c| to_priority(c))
        .sum();
}

fn part2(rucksacks: &Vec<String>) -> i32 {
    return rucksacks
        .chunks_exact(3)
        .map(|r| common_item(r))
        .map(|c| to_priority(c))
        .sum();
}

pub fn run() {
    let rucksacks = read_and_convert("data/day03.txt", |s| Some(s.to_string()));

    println!("== Day 03 ==");
    println!("Part 1: {}", part1(&rucksacks));
    println!("Part 2: {}", part2(&rucksacks));
}

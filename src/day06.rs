use std::collections::HashSet;
use std::fs::read_to_string;

fn find_start(line: &String, size: usize) -> Option<usize> {
    for i in 0..line.len() - size {
        let set: HashSet<char> = line[i..i + size].chars().collect();
        if set.len() == size {
            return Some(i + size);
        }
    }

    None
}

pub fn run() {
    let buffer = match read_to_string("data/day06.txt") {
        Ok(contents) => contents.trim().to_string(),
        _ => "".to_string(),
    };

    println!("== Day 06 ==");

    match find_start(&buffer, 4) {
        Some(index) => println!("Part 1: {}", index),
        None => println!("Part 1 error"),
    }

    match find_start(&buffer, 14) {
        Some(index) => println!("Part 2: {}", index),
        None => println!("Part 2 error"),
    }
}

use crate::utils::read_to_vec;

const BASE: i64 = 5;

fn to_digit(char: char) -> i64 {
    match char {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => panic!(),
    }
}

fn to_decimal(line: &str) -> i64 {
    line.chars()
        .rev()
        .map(|c| to_digit(c))
        .enumerate()
        .map(|(i, d)| d * BASE.pow(i as u32))
        .sum()
}

fn to_snafu(decimal: i64, place: u32) -> Option<Vec<char>> {
    let value = BASE.pow(place);
    if decimal < -3 * value || decimal > 3 * value {
        return None;
    }

    if place == 0 {
        match decimal {
            _ if decimal == -2 * value => Some(vec!['=']),
            _ if decimal == -value => Some(vec!['-']),
            _ if decimal == 0 => Some(vec!['0']),
            _ if decimal == value => Some(vec!['1']),
            _ if decimal == 2 * value => Some(vec!['2']),
            _ => None,
        }
    } else {
        ['=', '-', '0', '1', '2'].into_iter().find_map(|c| {
            match to_snafu(decimal - to_digit(c) * value, place - 1) {
                Some(tail) => Some([vec![c], tail].concat()),
                None => None,
            }
        })
    }
}

fn part1() -> String {
    let target = read_to_vec("data/day25.txt", to_decimal).iter().sum();
    let mut place = 1;
    loop {
        if let Some(snafu) = to_snafu(target, place) {
            return snafu.iter().collect();
        }
        place += 1;
    }
}

pub fn run() {
    println!("== Day 25 ==");
    println!("Part 1: {}", part1());
}

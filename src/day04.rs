use super::utils::read_to_vec;

struct Range {
    start: i32,
    end: i32,
}

fn to_range(range: &str) -> Range {
    let parts: Vec<&str> = range.split("-").collect();
    let start: i32 = parts[0].trim().parse().unwrap();
    let end: i32 = parts[1].trim().parse().unwrap();
    Range { start, end }
}

fn to_pair(line: &str) -> (Range, Range) {
    let parts: Vec<&str> = line.split(",").collect();
    (to_range(parts[0]), to_range(parts[1]))
}

fn count_if(pairs: &Vec<(Range, Range)>, predicate: fn(&Range, &Range) -> bool) -> usize {
    pairs.iter().filter(|(a, b)| predicate(a, b)).count()
}

fn contains(lhs: &Range, rhs: &Range) -> bool {
    (lhs.start >= rhs.start && lhs.end <= rhs.end) || (rhs.start >= lhs.start && rhs.end <= lhs.end)
}

fn overlaps(lhs: &Range, rhs: &Range) -> bool {
    rhs.end >= lhs.start && rhs.start <= lhs.end
}

pub fn run() {
    let pairs = read_to_vec("data/day04.txt", to_pair);

    println!("== Day 04 ==");
    println!("Part 1: {}", count_if(&pairs, contains));
    println!("Part 2: {}", count_if(&pairs, overlaps));
}

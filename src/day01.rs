use crate::utils::read_to_vec;

fn to_item(line: &str) -> Option<i32> {
    match line.trim() {
        "" => None,
        val => Some(val.parse().unwrap()),
    }
}

pub fn run() {
    let items = read_to_vec("./data/day01.txt", to_item);

    let mut calories: Vec<i32> = vec![];
    let mut sum = 0i32;
    for item in items {
        match item {
            Some(value) => sum += value,
            None => {
                calories.push(sum);
                sum = 0
            }
        }
    }
    calories.push(sum);
    calories.sort_by(|a, b| b.cmp(a));

    let max_values = &calories[0..3];
    println!("== Day 01 ==");
    println!("Part 1: {}", max_values[0]);
    println!("Part 2: {}", max_values[0] + max_values[1] + max_values[2]);
}

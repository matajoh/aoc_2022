use super::utils::read_lines;

pub fn run() {
    let mut calories: Vec<i32> = vec![];

    if let Ok(lines) = read_lines("./data/day01.txt") {
        let mut sum = 0i32;
        for line in lines {
            if let Ok(ip) = line {
                if ip.trim().is_empty() {
                    calories.push(sum);
                    sum = 0;
                } else {
                    let count: i32 = ip.trim().parse().unwrap();
                    sum += count;
                }
            }
        }
        calories.push(sum);
    }

    calories.sort_by(|a, b| b.cmp(a));

    let max_values = &calories[0..3];
    println!("== Day 01 ==");
    println!("Part 1: {}", max_values[0]);
    println!("Part 2: {}", max_values[0] + max_values[1] + max_values[2]);
}

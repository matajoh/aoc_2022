use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

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
    println!("Part 1: {}", max_values[0]);
    println!("Part 2: {}", max_values[0] + max_values[1] + max_values[2]);
}

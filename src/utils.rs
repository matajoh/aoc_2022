use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_and_convert<P, T>(filename: P, convert: fn(&str) -> Option<T>) -> Vec<T>
where
    P: AsRef<Path>,
{
    let mut result: Vec<T> = vec![];
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                if let Some(item) = convert(&ip) {
                    result.push(item);
                }
            }
        }
    }

    return result;
}

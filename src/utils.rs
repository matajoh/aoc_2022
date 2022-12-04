use std::fs::read_to_string;

pub fn read_to_vec<T>(filename: &str, convert: fn(&str) -> T) -> Vec<T> {
    match read_to_string(filename) {
        Ok(contents) => contents
            .lines()
            .into_iter()
            .map(|line| convert(line))
            .collect(),
        _ => vec![],
    }
}

pub fn read_some_to_vec<T>(filename: &str, convert: fn(&str) -> Option<T>) -> Vec<T> {
    match read_to_string(filename) {
        Ok(contents) => contents
            .lines()
            .into_iter()
            .map(|line| convert(line))
            .filter_map(|line| line)
            .collect(),
        _ => vec![],
    }
}

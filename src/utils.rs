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

pub fn is_match<T: Eq>(values: &Vec<T>, i: usize, j: usize, length: usize) -> bool {
    (i..i + length)
        .into_iter()
        .zip((j..j + length).into_iter())
        .all(|(i, j)| values[i] == values[j])
}

pub fn find_next<T: Eq>(
    values: &Vec<T>,
    pattern_start: usize,
    pattern_length: usize,
) -> Option<usize> {
    for i in pattern_start + pattern_length..values.len() - pattern_length {
        if is_match(values, pattern_start, i, pattern_length) {
            return Some(i);
        }
    }

    None
}

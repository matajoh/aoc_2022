use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Debug,
    fs::read_to_string,
    hash::Hash,
    ops::Add,
};

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

pub trait SearchInfo<T, I> {
    fn neighbors(&self, node: &T) -> Vec<T>;
    fn heuristic(&self, node: &T) -> I;
    fn distance(&self, start: &T, end: &T) -> I;
    fn start(&self) -> T;
    fn is_goal(&self, node: &T) -> bool;
    fn infinity(&self) -> I;
    fn zero(&self) -> I;
}

struct Ranking<T, I: Ord + PartialOrd>(T, I);

impl<T, I: Ord + PartialOrd> Ord for Ranking<T, I> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.1.cmp(&self.1)
    }
}

impl<T, I: Ord + PartialOrd> PartialOrd for Ranking<T, I> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T, I: Ord + PartialOrd> Eq for Ranking<T, I> {}

impl<T, I: Ord + PartialOrd> PartialEq for Ranking<T, I> {
    fn eq(&self, other: &Self) -> bool {
        match self.cmp(other) {
            Ordering::Equal => true,
            _ => false,
        }
    }
}

pub fn reconstruct_path<T: Eq + Hash + Copy>(search_result: &(HashMap<T, T>, T)) -> Vec<T> {
    let (came_from, end) = search_result;
    let mut path = vec![*end];
    let mut current = *end;
    while came_from.contains_key(&current) {
        current = came_from[&current];
        path.push(current);
    }
    path.reverse();
    path
}

pub fn astar_search<
    T: Eq + Hash + Copy,
    I: Debug + Copy + Ord + PartialOrd + Add<Output = I>,
    S: SearchInfo<T, I>,
>(
    info: &S,
) -> Option<(HashMap<T, T>, T)> {
    let mut open_set = HashSet::new();
    open_set.insert(info.start());

    let mut came_from = HashMap::new();

    let mut g_score = HashMap::new();
    g_score.insert(info.start(), info.zero());

    let mut f_score = BinaryHeap::new();
    f_score.push(Ranking(info.start(), info.heuristic(&info.start())));

    while !open_set.is_empty() {
        let current = f_score.pop().unwrap().0;
        if info.is_goal(&current) {
            return Some((came_from, current));
        }

        open_set.remove(&current);
        for neighbor in info.neighbors(&current) {
            let tentative_g_score = g_score[&current] + info.distance(&current, &neighbor);
            let current_g_score = match g_score.get(&neighbor) {
                Some(g_score) => *g_score,
                None => info.infinity(),
            };
            if tentative_g_score < current_g_score {
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g_score);
                f_score.push(Ranking(
                    neighbor.clone(),
                    tentative_g_score + info.heuristic(&neighbor),
                ));
                if !open_set.contains(&neighbor) {
                    open_set.insert(neighbor);
                }
            }
        }
    }

    None
}

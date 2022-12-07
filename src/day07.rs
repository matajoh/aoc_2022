use std::collections::HashMap;

use super::utils::read_to_vec;

enum Item {
    Directory(String, Vec<String>),
    File(String, i64),
}

impl Item {
    fn size(&self, items: &HashMap<String, Item>) -> i64 {
        match self {
            Item::File(_, length) => *length,
            Item::Directory(_, children) => children.iter().map(|i| items[i].size(items)).sum(),
        }
    }
}

fn join_path(dir: &String, name: &str) -> String {
    match dir.as_str() {
        "" => name.to_string(),
        prefix if prefix.ends_with("/") => [prefix, name].join(""),
        prefix => [prefix, name].join("/"),
    }
}

fn dir_name(dir: &String) -> String {
    match dir.rfind('/') {
        Some(0) => "/".to_string(),
        Some(index) => dir[0..index].to_string(),
        None => "".to_string(),
    }
}

fn process(lines: Vec<String>) -> HashMap<String, Item> {
    let mut items = HashMap::new();
    let mut dir = "".to_string();
    let mut files: Vec<String> = vec![];
    for line in lines {
        let parts: Vec<&str> = line.trim().split(" ").collect();
        match parts[..] {
            ["$", "cd", ".."] => {
                if files.len() > 0 {
                    items.insert(dir.clone(), Item::Directory(dir.clone(), files.clone()));
                    files.clear();
                }
                dir = dir_name(&dir);
            }
            ["$", "cd", dir_name] => {
                if files.len() > 0 {
                    items.insert(dir.clone(), Item::Directory(dir.clone(), files.clone()));
                    files.clear();
                }
                dir = join_path(&dir, dir_name)
            }
            ["$", "ls"] => files.clear(),
            ["dir", name] => {
                let key = join_path(&dir, name);
                items.insert(key.clone(), Item::Directory(key.clone(), vec![]));
                files.push(key);
            }
            [digits, name] => {
                let key = join_path(&dir, name);
                let length: i64 = digits.parse().unwrap();
                items.insert(key.clone(), Item::File(key.clone(), length));
                files.push(key);
            }
            _ => panic!("Error"),
        }
    }

    if files.len() > 0 {
        items.insert(dir.clone(), Item::Directory(dir.clone(), files.clone()));
    }

    items
}

fn part1(items: &HashMap<String, Item>) -> i64 {
    items
        .iter()
        .map(|(_, item)| match item {
            Item::Directory(_, _) => Some(item.size(items)),
            Item::File(_, _) => None,
        })
        .filter_map(|d| d)
        .filter(|s| *s <= 100000)
        .sum()
}

fn part2(items: &HashMap<String, Item>) -> i64 {
    let free_space = 70000000 - items["/"].size(items);
    let target = 30000000 - free_space;
    items
        .iter()
        .map(|(_, item)| match item {
            Item::Directory(_, _) => Some(item.size(items)),
            Item::File(_, _) => None,
        })
        .filter_map(|d| d)
        .filter(|s| *s >= target)
        .min()
        .unwrap()
}

pub fn run() {
    let lines: Vec<String> = read_to_vec("data/day07.txt", |a| a.to_string());
    let items = process(lines);

    println!("== Day 07 ==");
    println!("Part 1: {}", part1(&items));
    println!("Part 2: {}", part2(&items))
}

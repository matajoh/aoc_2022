use super::utils::read_to_vec;
use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq)]
enum Item {
    Rock,
    Sand,
    FallingSand,
    Start,
}

type Point = (i32, i32);

fn to_rocks(line: &str) -> Vec<(Point, Item)> {
    line.trim()
        .split(" -> ")
        .map(|p| {
            p.split(',')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|a| (a[0], a[1]))
        .collect::<Vec<(i32, i32)>>()
        .windows(2)
        .into_iter()
        .flat_map(|pair| match (pair[0], pair[1]) {
            ((x0, y0), (x1, y1)) if x0 == x1 && y0 <= y1 => (y0..=y1)
                .map(|y| ((x0, y), Item::Rock))
                .collect::<Vec<(Point, Item)>>(),
            ((x0, y0), (x1, y1)) if x0 == x1 => (y1..=y0)
                .map(|y| ((x0, y), Item::Rock))
                .collect::<Vec<(Point, Item)>>(),
            ((x0, y0), (x1, y1)) if y0 == y1 && x0 <= x1 => (x0..=x1)
                .map(|x| ((x, y0), Item::Rock))
                .collect::<Vec<(Point, Item)>>(),
            ((x0, y0), (x1, y1)) if y0 == y1 => (x1..=x0)
                .map(|x| ((x, y0), Item::Rock))
                .collect::<Vec<(Point, Item)>>(),
            _ => panic!("invalid rock line"),
        })
        .collect()
}

fn read_rocks() -> HashMap<Point, Item> {
    read_to_vec("data/day14.txt", to_rocks)
        .into_iter()
        .flatten()
        .chain([((500, 0), Item::Start)])
        .collect()
}

fn get_bottom(items: &HashMap<Point, Item>) -> i32 {
    items
        .keys()
        .fold(0, |max, (_, y)| if *y > max { *y } else { max })
}

fn down(p: &Point) -> Point {
    (p.0, p.1 + 1)
}

fn down_left(p: &Point) -> Point {
    (p.0 - 1, p.1 + 1)
}

fn down_right(p: &Point) -> Point {
    (p.0 + 1, p.1 + 1)
}

fn get_item(items: &HashMap<Point, Item>, p: &Point, floor: Option<i32>) -> Option<Item> {
    match (items.get(&p), floor) {
        (Some(item), _) => Some(*item),
        (None, None) => None,
        (None, Some(floor)) if p.1 == floor => Some(Item::Rock),
        _ => None,
    }
}

fn add_sand(items: &HashMap<Point, Item>, bottom: i32, floor: Option<i32>) -> Option<Point> {
    let mut p: Point = (500, 0);
    if items[&p] == Item::Sand {
        return None;
    }

    let mut sand = Item::FallingSand;
    while p.1 < bottom && sand == Item::FallingSand {
        let (d, dl, dr) = (down(&p), down_left(&p), down_right(&p));
        (p, sand) = match get_item(items, &d, floor) {
            None => (d, Item::FallingSand),
            Some(_) => match get_item(items, &dl, floor) {
                None => (dl, Item::FallingSand),
                Some(_) => match get_item(items, &dr, floor) {
                    None => (dr, Item::FallingSand),
                    Some(_) => (p, Item::Sand),
                },
            },
        }
    }
    match sand {
        Item::Sand => Some(p),
        _ => None,
    }
}

fn part1(rocks: &HashMap<Point, Item>) -> usize {
    let bottom = get_bottom(rocks);
    let mut items: HashMap<Point, Item> = HashMap::new();
    for (p, i) in rocks {
        items.insert(*p, *i);
    }
    let mut count = 0;
    while let Some(p) = add_sand(&items, bottom, None) {
        items.insert(p, Item::Sand);
        count += 1
    }
    count
}

fn part2(rocks: &HashMap<Point, Item>) -> usize {
    let bottom = get_bottom(rocks) + 2;
    let mut items: HashMap<Point, Item> = HashMap::new();
    for (p, i) in rocks {
        items.insert(*p, *i);
    }
    let mut count = 0;
    while let Some(p) = add_sand(&items, bottom, Some(bottom)) {
        items.insert(p, Item::Sand);
        count += 1
    }
    count
}

pub fn run() {
    let rocks = read_rocks();
    println!("== Day 14 ==");
    println!("Part 1: {}", part1(&rocks));
    println!("Part 2: {}", part2(&rocks))
}

use super::utils::read_to_vec;
use std::cmp::Ordering;

struct Sensor {
    x: i64,
    y: i64,
    r: i64,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Range {
    end: i64,
    start: i64,
}

#[derive(Debug)]
enum Line {
    Positive(i64),
    Negative(i64),
}

impl Line {
    fn intersect(&self, other: &Line) -> Option<(i64, i64)> {
        match (self, other) {
            (Line::Positive(c0), Line::Negative(c1)) | (Line::Negative(c1), Line::Positive(c0)) => {
                let x = -(c1 - c0) / 2;
                let y = c0 - x;
                Some((x, y))
            }
            _ => None,
        }
    }
}

impl Sensor {
    fn intersect_y(&self, y: i64) -> Option<Range> {
        match (self.y - y).abs() {
            dy if dy > self.r => None,
            dy => {
                let dx = self.r - dy;
                Some(Range {
                    start: self.x - dx,
                    end: self.x + dx + 1,
                })
            }
        }
    }

    fn gap_to(&self, other: &Sensor) -> i64 {
        let dx = (self.x - other.x).abs();
        let dy = (self.y - other.y).abs();
        dx + dy - self.r - other.r
    }

    fn separator(&self, other: &Sensor) -> Option<Line> {
        match (self.x.cmp(&other.x), self.y.cmp(&other.y)) {
            (Ordering::Less, Ordering::Less) => {
                let x = self.x + self.r + 1;
                Some(Line::Positive(self.y + x))
            }
            (Ordering::Greater, Ordering::Greater) => {
                let x = self.x - self.r - 1;
                Some(Line::Positive(self.y + x))
            }
            (Ordering::Less, Ordering::Greater) => {
                let x = self.x + self.r + 1;
                Some(Line::Negative(self.y - x))
            }
            (Ordering::Greater, Ordering::Less) => {
                let x = self.x - self.r - 1;
                Some(Line::Negative(self.y - x))
            }
            _ => None,
        }
    }
}

impl Range {
    fn intersects(&self, other: &Range) -> bool {
        other.end > self.start && self.end >= other.start
    }

    fn union_with(&self, other: &Range) -> Option<Range> {
        if self.intersects(other) {
            Some(Range {
                start: self.start.min(other.start),
                end: self.end.max(other.end),
            })
        } else {
            None
        }
    }

    fn intersect_with(&self, other: &Range) -> Option<Range> {
        if self.intersects(other) {
            Some(Range {
                start: self.start.max(other.start),
                end: self.end.min(other.end),
            })
        } else {
            None
        }
    }

    fn len(&self) -> i64 {
        self.end - self.start - 1
    }
}

fn to_sensor(line: &str) -> Sensor {
    let parts = line.split(&[' ', '=', ',', ':']).collect::<Vec<&str>>();
    let x = parts[3].parse::<i64>().unwrap();
    let y = parts[6].parse::<i64>().unwrap();
    let bx = parts[13].parse::<i64>().unwrap();
    let by = parts[16].parse::<i64>().unwrap();
    let r = (x - bx).abs() + (y - by).abs();
    Sensor { x, y, r }
}

fn size(ranges: &Vec<Range>) -> i64 {
    ranges.iter().map(|r| r.len()).sum()
}

fn empty_ranges(sensors: &Vec<Sensor>, y: i64, bounds: &Option<Range>) -> Vec<Range> {
    let mut ranges = sensors
        .iter()
        .filter_map(|s| match (s.intersect_y(y), bounds) {
            (Some(r0), Some(r1)) => r0.intersect_with(&r1),
            (Some(r0), None) => Some(r0),
            _ => None,
        })
        .collect::<Vec<Range>>();

    ranges.sort_unstable();

    while ranges.len() > 1 {
        let r1 = ranges.pop().unwrap();
        let r0 = ranges.pop().unwrap();
        if let Some(r) = r0.union_with(&r1) {
            ranges.push(r)
        } else {
            ranges.push(r0);
            break;
        }
    }

    ranges
}

fn part1(sensors: &Vec<Sensor>, y: i64) -> i64 {
    let empty = empty_ranges(sensors, y, &None);
    size(&empty)
}

fn part2(sensors: &Vec<Sensor>) -> i64 {
    let lines = (0..sensors.len())
        .flat_map(|i| {
            (i + 1..sensors.len()).filter_map(move |j| match sensors[i].gap_to(&sensors[j]) {
                2 => sensors[i].separator(&sensors[j]),
                _ => None,
            })
        })
        .collect::<Vec<Line>>();

    let (x, y) = lines[0].intersect(&lines[1]).unwrap();
    x * 4000000 + y
}

pub fn run() {
    let sensors = read_to_vec("data/day15.txt", to_sensor);
    println!("== Day 15 ==");
    println!("Part 1: {}", part1(&sensors, 10));
    println!("Part 2: {}", part2(&sensors));
}

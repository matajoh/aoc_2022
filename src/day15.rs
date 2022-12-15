use super::utils::read_to_vec;

#[derive(Debug)]
struct Sensor {
    x: i64,
    y: i64,
    r: i64,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
struct Range {
    start: i64,
    end: i64,
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
}

impl Range {
    fn intersects(&self, other: &Range) -> bool {
        other.end > self.start && self.end >= other.start
    }

    fn union_with(&self, other: &Range) -> Range {
        Range {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
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

fn empty_ranges(
    sensors: &Vec<Sensor>,
    intersect: fn(&Sensor, i64) -> Option<Range>,
    value: i64,
) -> Vec<Range> {
    let mut ranges = sensors
        .iter()
        .filter_map(|s| intersect(s, value))
        .collect::<Vec<Range>>();

    ranges.sort();

    let mut i = 0;
    while i < ranges.len() - 1 {
        if ranges[i].intersects(&ranges[i + 1]) {
            let r = ranges.remove(i + 1);
            ranges[i] = ranges[i].union_with(&r);
        } else {
            i += 1;
        }
    }

    ranges
}

fn part1(sensors: &Vec<Sensor>, y: i64) -> i64 {
    let empty = empty_ranges(sensors, |s, v| s.intersect_y(v), y);
    size(&empty)
}

fn part2(sensors: &Vec<Sensor>, size: i64) -> i64 {
    let bounds = Range {
        start: 0,
        end: size,
    };

    let (x, y) = (0..size)
        .map(|y| {
            (
                y,
                empty_ranges(sensors, |s, v| s.intersect_y(v), y)
                    .iter()
                    .filter_map(|r| r.intersect_with(&bounds))
                    .collect::<Vec<Range>>(),
            )
        })
        .filter(|(_, rs)| rs.len() == 2)
        .map(|(y, rs)| (rs[0].end, y))
        .next()
        .unwrap();
    x * 4000000 + y
}

pub fn run() {
    let sensors = read_to_vec("data/day15.txt", to_sensor);
    println!("== Day 15 ==");
    println!("Part 1: {}", part1(&sensors, 10));
    println!("Part 2: {}", part2(&sensors, 4000000));
}

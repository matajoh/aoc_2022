use super::utils::read_to_vec;

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

    fn midpoint(&self, other: &Sensor) -> (i64, i64) {
        let dx = (other.x - self.x) as f32;
        let dy = (other.y - self.y) as f32;
        let length = (dx * dx + dy * dy).sqrt();
        let x = (self.x as f32) + (dx * (self.r as f32)) / length;
        let y = (self.y as f32) + (dy * (self.r as f32)) / length;
        (x as i64, y as i64)
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

fn get_explore_range(sensors: &Vec<Sensor>) -> (i64, i64) {
    let mut gaps = (0..sensors.len())
        .flat_map(|i| {
            (i + 1..sensors.len()).filter_map(move |j| match sensors[i].gap_to(&sensors[j]) {
                2 => Some(sensors[i].midpoint(&sensors[j]).1),
                _ => None,
            })
        })
        .collect::<Vec<i64>>();
    gaps.sort();
    (gaps[0], gaps[gaps.len() - 1])
}

fn part2(sensors: &Vec<Sensor>, size: i64) -> i64 {
    let bounds = Some(Range {
        start: 0,
        end: size,
    });

    let (start, end) = get_explore_range(sensors);

    let (x, y) = (start..end)
        .map(|y| (y, empty_ranges(sensors, y, &bounds)))
        .filter(|(_, rs)| rs.len() > 1)
        .map(|(y, rs)| (rs[rs.len() - 1].end, y))
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

use crate::utils::read_to_vec;

fn to_final(numbers: &Vec<i32>, indices: &Vec<i32>) -> Vec<i32> {
    let mut result = vec![0; numbers.len()];
    for n in 0..numbers.len() {
        result[indices[n] as usize] = numbers[n]
    }
    result
}

fn mix(numbers: &Vec<i32>) -> Vec<i32> {
    let count = numbers.len() as i32;
    let mut indices = (0..count).collect::<Vec<i32>>();
    for n in 0..numbers.len() {
        let index = indices[n] + numbers[n] % count;
        let (start, end, inc) = match index {
            _ if index >= count => (index - count, indices[n], 1),
            _ if index < 0 => (indices[n], count + index - 1, -1),
            _ if numbers[n] > 0 => (indices[n], index, -1),
            _ if numbers[n] < 0 => (index, indices[n], 1),
            _ => panic!("invalid delta"),
        };

        println!("{}..={} => {}", start, end, inc);

        for i in indices.iter_mut() {
            if *i > start && *i <= end {
                *i += inc
            }
        }

        indices[n] = index;

        for i in indices.iter_mut() {
            if *i < 0 {
                *i += count
            } else if *i >= count {
                *i -= count
            }
        }

        println!("{:?}", to_final(numbers, &indices))
    }

    to_final(numbers, &indices)
}

fn part1(numbers: &Vec<i32>) -> i32 {
    let mixed = mix(numbers);
    println!("{:?}", mixed);
    0
}

pub fn run() {
    let numbers = read_to_vec("data/day20.txt", |s| s.trim().parse::<i32>().unwrap());
    println!("== Day 20 ==");
    println!("Part 1: {}", part1(&numbers))
}

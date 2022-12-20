use crate::utils::read_to_vec;

#[derive(Debug)]
struct Node {
    delta: i64,
    prev: usize,
    next: usize,
}

fn to_final(nodes: &Vec<Node>) -> Vec<i64> {
    let mut result = vec![0];
    let start = nodes.iter().position(|n| n.delta == 0).unwrap();
    let mut current = nodes[start].next;
    for _ in 1..nodes.len() {
        result.push(nodes[current].delta);
        current = nodes[current].next;
    }

    result
}

fn mix(nodes: &mut Vec<Node>) {
    let count = nodes.len() as i64 - 1;
    for n in 0..nodes.len() {
        let delta = nodes[n].delta % count;
        if delta == 0 {
            continue;
        }

        let mut prev = nodes[n].prev;
        let mut next = nodes[n].next;
        nodes[prev].next = next;
        nodes[next].prev = prev;

        if delta < 0 {
            for _ in delta..0 {
                prev = nodes[prev].prev
            }
        } else {
            for _ in 0..delta {
                prev = nodes[prev].next
            }
        }

        next = nodes[prev].next;
        nodes[n].next = next;
        nodes[next].prev = n;
        nodes[n].prev = prev;
        nodes[prev].next = n;
    }
}

fn part1() -> i64 {
    let mut nodes = read_nodes();
    mix(&mut nodes);
    let mixed = to_final(&nodes);
    [1000, 2000, 3000]
        .into_iter()
        .map(|i| mixed[i % mixed.len()])
        .sum()
}

fn part2() -> i64 {
    let mut nodes = read_nodes();
    for n in nodes.iter_mut() {
        n.delta *= 811589153;
    }

    for _ in 0..10 {
        mix(&mut nodes);
    }
    let mixed = to_final(&nodes);
    [1000, 2000, 3000]
        .into_iter()
        .map(|i| mixed[i % mixed.len()])
        .sum()
}

fn read_nodes() -> Vec<Node> {
    let deltas = read_to_vec("data/day20.txt", |s| s.trim().parse::<i64>().unwrap());
    let mut nodes = deltas
        .into_iter()
        .map(|d| Node {
            delta: d,
            prev: 0,
            next: 0,
        })
        .collect::<Vec<Node>>();

    for i in 0..nodes.len() {
        let (prev, next) = match i {
            0 => (nodes.len() - 1, 1),
            _ if i == nodes.len() - 1 => (nodes.len() - 2, 0),
            _ => (i - 1, i + 1),
        };
        nodes[i].prev = prev;
        nodes[i].next = next;
    }

    nodes
}

pub fn run() {
    println!("== Day 20 ==");
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2())
}

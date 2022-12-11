use std::fs::read_to_string;

#[derive(Copy, Clone)]
enum Arg {
    Old,
    Constant(usize),
}

#[derive(Copy, Clone)]
enum Operation {
    Add(Arg, Arg),
    Multiply(Arg, Arg),
}

impl Operation {
    fn get_result(&self, old: usize) -> usize {
        match self {
            Operation::Add(Arg::Old, Arg::Old) => old + old,
            Operation::Add(Arg::Old, Arg::Constant(value))
            | Operation::Add(Arg::Constant(value), Arg::Old) => old + value,
            Operation::Multiply(Arg::Old, Arg::Old) => old * old,
            Operation::Multiply(Arg::Old, Arg::Constant(value))
            | Operation::Multiply(Arg::Constant(value), Arg::Old) => old * value,
            _ => panic!("invalid operation"),
        }
    }
}

type Test = (usize, usize, usize);

struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test: Test,
    inspection_count: usize,
}

enum WorryManagement {
    Decrease,
    Modulo(usize),
}

fn inspect_and_throw(monkeys: &mut Vec<Monkey>, index: usize, worry_management: &WorryManagement) {
    monkeys.push(Monkey {
        items: vec![],
        operation: monkeys[index].operation,
        test: monkeys[index].test,
        inspection_count: monkeys[index].inspection_count + monkeys[index].items.len(),
    });
    let mut monkey = monkeys.swap_remove(index);
    for item in monkey.items.into_iter() {
        let worry = match (monkey.operation.get_result(item), worry_management) {
            (worry, WorryManagement::Decrease) => worry / 3,
            (worry, WorryManagement::Modulo(value)) => worry % value,
        };
        let index = match monkey.test {
            (divisor, m_true, _) if worry % divisor == 0 => m_true,
            (_, _, m_false) => m_false,
        };
        monkeys[index].items.push(worry);
        monkey.inspection_count += 1
    }
}

fn get_last(line: &str) -> usize {
    line.trim().split(" ").last().unwrap().parse().unwrap()
}

fn parse_items(line: &str) -> Vec<usize> {
    line[18..]
        .trim()
        .split(",")
        .map(|i| i.trim().parse().unwrap())
        .collect()
}

fn parse_op(line: &str) -> Operation {
    let parts: Vec<&str> = line.trim().split(" ").collect();
    let arg0 = match parts[3] {
        "old" => Arg::Old,
        constant => Arg::Constant(constant.parse().unwrap()),
    };
    let arg1 = match parts[5] {
        "old" => Arg::Old,
        constant => Arg::Constant(constant.parse().unwrap()),
    };
    match parts[4] {
        "+" => Operation::Add(arg0, arg1),
        "*" => Operation::Multiply(arg0, arg1),
        _ => panic!("invalid operator"),
    }
}

fn parse_test(lines: &[&str]) -> Test {
    (get_last(lines[0]), get_last(lines[1]), get_last(lines[2]))
}

fn to_monkey(lines: &[&str]) -> Monkey {
    Monkey {
        items: parse_items(lines[1]),
        operation: parse_op(lines[2]),
        test: parse_test(&lines[3..6]),
        inspection_count: 0,
    }
}

fn parse_monkeys() -> Vec<Monkey> {
    match read_to_string("data/day11.txt") {
        Ok(contents) => {
            let lines: Vec<&str> = contents.lines().collect();
            lines.chunks(7).map(|x| to_monkey(&x[..6])).collect()
        }
        _ => vec![],
    }
}

fn monkey_business(
    mut monkeys: Vec<Monkey>,
    num_rounds: usize,
    worry_management: WorryManagement,
) -> usize {
    for _ in 0..num_rounds {
        for i in 0..monkeys.len() {
            inspect_and_throw(&mut monkeys, i, &worry_management)
        }
    }
    monkeys.sort_by(|a, b| b.inspection_count.cmp(&a.inspection_count));
    monkeys[0].inspection_count * monkeys[1].inspection_count
}

fn part1() -> usize {
    let monkeys = parse_monkeys();
    monkey_business(monkeys, 20, WorryManagement::Decrease)
}

fn part2() -> usize {
    let monkeys = parse_monkeys();
    let modulo = monkeys
        .iter()
        .map(|m| m.test.0)
        .reduce(|a, b| a * b)
        .unwrap();
    monkey_business(monkeys, 10000, WorryManagement::Modulo(modulo))
}

pub fn run() {
    println!("== Day 11 ==");
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2())
}

use std::collections::HashMap;

use crate::utils::read_to_vec;

enum Expression {
    Literal(i64),
    Add(String, String),
    Subtract(String, String),
    Multiply(String, String),
    Divide(String, String),
}

fn to_monkey(line: &str) -> (String, Expression) {
    let parts = line.trim().split([':', ' ']).collect::<Vec<&str>>();
    if parts.len() == 3 {
        (
            parts[0].to_string(),
            Expression::Literal(parts[2].parse().unwrap()),
        )
    } else {
        match parts[3] {
            "+" => (
                parts[0].to_string(),
                Expression::Add(parts[2].to_string(), parts[4].to_string()),
            ),
            "-" => (
                parts[0].to_string(),
                Expression::Subtract(parts[2].to_string(), parts[4].to_string()),
            ),
            "*" => (
                parts[0].to_string(),
                Expression::Multiply(parts[2].to_string(), parts[4].to_string()),
            ),
            "/" => (
                parts[0].to_string(),
                Expression::Divide(parts[2].to_string(), parts[4].to_string()),
            ),
            _ => panic!("Invalid input"),
        }
    }
}

fn eval(monkeys: &HashMap<String, Expression>, name: &str) -> i64 {
    match &monkeys[name] {
        Expression::Literal(value) => *value,
        Expression::Add(lhs, rhs) => eval(monkeys, &lhs) + eval(monkeys, &rhs),
        Expression::Subtract(lhs, rhs) => eval(monkeys, &lhs) - eval(monkeys, &rhs),
        Expression::Multiply(lhs, rhs) => eval(monkeys, &lhs) * eval(monkeys, &rhs),
        Expression::Divide(lhs, rhs) => eval(monkeys, &lhs) / eval(monkeys, &rhs),
    }
}

fn has_child(monkeys: &HashMap<String, Expression>, name: &str, child: &str) -> bool {
    match &monkeys[name] {
        _ if name == child => true,
        Expression::Literal(_) => false,
        Expression::Add(lhs, rhs)
        | Expression::Subtract(lhs, rhs)
        | Expression::Multiply(lhs, rhs)
        | Expression::Divide(lhs, rhs) => {
            has_child(monkeys, lhs, child) || has_child(monkeys, rhs, child)
        }
    }
}

fn make_equal(monkeys: &HashMap<String, Expression>, name: &str, target: i64) -> i64 {
    if name == "humn" {
        return target;
    }

    let (sub_target, child) = match &monkeys[name] {
        Expression::Add(lhs, rhs)
        | Expression::Subtract(lhs, rhs)
        | Expression::Multiply(lhs, rhs)
        | Expression::Divide(lhs, rhs) => {
            if has_child(monkeys, lhs, "humn") {
                (eval(monkeys, rhs), lhs)
            } else {
                (eval(monkeys, lhs), rhs)
            }
        }
        _ => panic!(),
    };

    match &monkeys[name] {
        Expression::Add(_, _) => make_equal(monkeys, child, target - sub_target),
        Expression::Subtract(lhs, _) if lhs == child => {
            make_equal(monkeys, child, target + sub_target)
        }
        Expression::Subtract(_, rhs) if rhs == child => {
            make_equal(monkeys, child, sub_target - target)
        }
        Expression::Multiply(_, _) => make_equal(monkeys, child, target / sub_target),
        Expression::Divide(lhs, _) if lhs == child => {
            make_equal(monkeys, child, target * sub_target)
        }
        Expression::Divide(_, rhs) if rhs == child => {
            make_equal(monkeys, child, sub_target / target)
        }
        _ => panic!(),
    }
}

fn part2(monkeys: &HashMap<String, Expression>) -> i64 {
    let (target, child) = match &monkeys["root"] {
        Expression::Add(lhs, rhs)
        | Expression::Subtract(lhs, rhs)
        | Expression::Multiply(lhs, rhs)
        | Expression::Divide(lhs, rhs) => {
            if has_child(monkeys, lhs, "humn") {
                (eval(monkeys, rhs), lhs)
            } else {
                (eval(monkeys, lhs), rhs)
            }
        }
        _ => panic!("Invalid"),
    };
    make_equal(monkeys, child, target)
}

pub fn run() {
    let monkeys: HashMap<String, Expression> = read_to_vec("data/day21.txt", to_monkey)
        .into_iter()
        .collect();
    println!("== Day 21 ==");
    println!("Part 1: {}", eval(&monkeys, "root"));
    println!("Part 2: {}", part2(&monkeys));
}

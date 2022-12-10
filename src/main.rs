mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod utils;

use std::env;

fn run_all() {
    day01::run();
    day02::run();
    day03::run();
    day04::run();
    day05::run();
    day06::run();
    day07::run();
    day08::run();
    day09::run();
    day10::run();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        match args[1].as_str() {
            "1" => day01::run(),
            "2" => day02::run(),
            "3" => day03::run(),
            "4" => day04::run(),
            "5" => day05::run(),
            "6" => day06::run(),
            "7" => day07::run(),
            "8" => day08::run(),
            "9" => day09::run(),
            "10" => day10::run(),
            "all" => run_all(),
            _ => println!("Unrecognized option: {}", args[1]),
        }
    } else {
        run_all()
    }
}

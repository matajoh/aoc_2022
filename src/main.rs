mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod utils;

use std::env;

fn run_all() {
    day01::run();
    day02::run();
    day03::run();
    day04::run();
    day05::run();
    day06::run();
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
            "all" => run_all(),
            _ => println!("Unrecognized option: {}", args[1]),
        }
    } else {
        run_all()
    }
}

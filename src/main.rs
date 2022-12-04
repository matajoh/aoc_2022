mod day01;
mod day02;
mod day03;
mod day04;
mod utils;

use std::env;

fn run_all() {
    day01::run();
    day02::run();
    day03::run();
    day04::run();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 && args[1] == "all" {
        run_all();
    } else {
        day04::run();
    }
}

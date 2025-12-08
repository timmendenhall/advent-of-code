use advent_of_code::config::Config;
use core::num;
use std::cell::RefCell;
use std::env;
use std::fs;
use std::process;

#[cfg(test)]
mod tests;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    do_puzzle(config)
}

fn do_puzzle(config: Config) {
    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    let password = match config.strategy.as_str() {
        "part-a" => part_a_strategy(contents),
        "part-b" => part_b_strategy(contents),
        _ => part_a_strategy(contents),
    };

    println!("Password is: {}", password);
}

fn part_a_strategy(input: String) -> i64 {
    0
}

fn part_b_strategy(input: String) -> i64 {
    1
}

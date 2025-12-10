use advent_of_code::config::Config;
use std::env;
use std::fs;
use std::process;

#[cfg(test)]
mod tests;

struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Vec3 {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

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
        "part-a" => do_part_a(contents),
        "part-b" => do_part_b(contents),
        _ => do_part_a(contents),
    };

    println!("Password is: {}", password);
}

fn do_part_a(contents: String) -> i64 {
    1
}

fn do_part_b(contents: String) -> i64 {
    2
}

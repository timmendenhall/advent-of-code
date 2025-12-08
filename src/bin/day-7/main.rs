use advent_of_code::config::Config;
use array2d::{Array2D, Error};
use std::env;
use std::fs;
use std::process;

#[cfg(test)]
mod tests;

#[derive(Clone)]
enum ManifoldStatus {
    Empty,
    Start,
    Splitter,
    Beam,
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
        "part-a" => part_a_strategy(contents),
        "part-b" => part_b_strategy(contents),
        _ => part_a_strategy(contents),
    };

    println!("Password is: {}", password);
}

fn part_a_strategy(input: String) -> i64 {
    let tachyon_manifold_diagram = build_manifold(input).unwrap();

    0
}

fn part_b_strategy(input: String) -> i64 {
    1
}

fn build_manifold(input: String) -> Result<Array2D<ManifoldStatus>, Error> {
    let rows = vec![vec![ManifoldStatus::Empty], vec![ManifoldStatus::Empty]];
    let from_rows = Array2D::from_rows(&rows)?;
    Ok(from_rows)
}

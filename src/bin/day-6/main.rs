use advent_of_code::config::Config;
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

fn multiply_set(set: &[i64]) -> i64 {
    set.iter().copied().reduce(|a, b| a * b).unwrap_or(0)
}

fn sum_set(set: &[i64]) -> i64 {
    set.iter().copied().reduce(|a, b| a + b).unwrap_or(0)
}

fn calculate_math_set(math_problems: Vec<RefCell<Vec<&str>>>) -> i64 {
    let mut total = 0;

    for set in math_problems {
        let mut parsed: Vec<i64> = Vec::new();

        for val in set.borrow().iter() {
            match *val {
                "*" => total += multiply_set(&parsed),
                "+" => total += sum_set(&parsed),
                _ => {
                    if let Ok(number) = val.trim().parse() {
                        parsed.push(number);
                    }
                }
            }
        }
    }

    total
}

fn part_a_strategy(input: String) -> i64 {
    let mut math_problems: Vec<RefCell<Vec<&str>>> = Vec::new();

    for line in input.lines() {
        let split: Vec<&str> = line.split_whitespace().collect();
        for (x, cell) in split.iter().enumerate() {
            if let Some(existing_set) = math_problems.get(x) {
                existing_set.borrow_mut().push(cell);
            } else {
                let new_set: Vec<&str> = vec![cell];
                math_problems.push(RefCell::from(new_set));
            }
        }
    }

    calculate_math_set(math_problems)
}

fn part_b_strategy(input: String) -> i64 {
    let mut math_problems: Vec<RefCell<Vec<&str>>> = Vec::new();
    let mut remaining_input = input.clone();

    for line in input.lines() {
        let split: Vec<&str> = line.split_whitespace().collect();
        for (x, cell) in split.iter().enumerate() {
            if let Some(existing_set) = math_problems.get(x) {
                existing_set.borrow_mut().push(cell);
            } else {
                let new_set: Vec<&str> = vec![cell];
                math_problems.push(RefCell::from(new_set));
            }
        }
    }

    calculate_math_set(math_problems)
}

use std::cell::RefCell;
use std::env;
use std::fs;
use std::process;

struct Config {
    file_path: String,
    strategy: fn(Vec<RefCell<Vec<&str>>>) -> i64,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let file_path = args[1].clone();
        let strategy_str = args[2].clone();

        let strategy = match strategy_str.as_str() {
            "part-a" => part_a_strategy,
            "part-b" => part_b_strategy,
            _ => part_a_strategy,
        };

        Ok(Config {
            file_path,
            strategy,
        })
    }
}

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

    let mut math_problems: Vec<RefCell<Vec<&str>>> = Vec::new();

    for line in contents.lines() {
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

    let password = (config.strategy)(math_problems);

    println!("Password is: {}", password);
}

fn multiply_set(set: &[i64]) -> i64 {
    set.iter().copied().reduce(|a, b| a * b).unwrap_or(0)
}

fn sum_set(set: &[i64]) -> i64 {
    set.iter().copied().reduce(|a, b| a + b).unwrap_or(0)
}

fn part_a_strategy(problems: Vec<RefCell<Vec<&str>>>) -> i64 {
    let mut password_addition = 0;

    for set in problems {
        let mut parsed: Vec<i64> = Vec::new();

        /*
            "part-a" => part_a_strategy,
            "part-b" => part_b_strategy,
            _ => part_a_strategy,
        */
        for val in set.borrow().iter() {
            match *val {
                "*" => password_addition += multiply_set(&parsed),
                "+" => password_addition += sum_set(&parsed),
                _ => {
                    if let Ok(number) = val.trim().parse() {
                        parsed.push(number);
                    }
                }
            }
        }
    }

    password_addition
}

fn part_b_strategy(problems: Vec<RefCell<Vec<&str>>>) -> i64 {
    let mut total_ids = 123;

    total_ids
}

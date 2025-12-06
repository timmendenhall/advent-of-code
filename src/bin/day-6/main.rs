use std::env;
use std::fs;
use std::process;

struct Config {
    file_path: String,
    strategy: fn(Vec<Vec<&str>>) -> i64,
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

    let mut math_problems: Vec<Vec<&str>> = Vec::new();

    for line in contents.lines() {
        let split: Vec<&str> = line.split_whitespace().collect();
        for (x, cell) in split.iter().enumerate() {
            // let parsed = cell.parse().unwrap();
            if let Some(a) = math_problems.get(x) {
                a.push(cell);
            } else {
                let mut b = Vec::new();
                b.push(cell);
                math_problems.push(b);
            }
        }
    }

    let password = (config.strategy)(math_problems);

    println!("Password is: {}", password);
}

fn part_a_strategy(problems: Vec<Vec<&str>>) -> i64 {
    let mut password_addition = 456;

    // for i in available_ingredient_ids {
    //     if is_id_in_range(i, &fresh_ingredient_id_ranges) {
    //         password_addition += 1;
    //     }
    // }

    password_addition
}

fn part_b_strategy(problems: Vec<Vec<&str>>) -> i64 {
    let mut total_ids = 123;

    total_ids
}

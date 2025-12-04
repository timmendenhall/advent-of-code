use std::env;
use std::fs;
use std::process;

struct Config {
    file_path: String,
    strategy: fn(i64) -> i64,
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    do_puzzle(config)
}

fn do_puzzle(config: Config) {
    let mut password = 0;

    let contents = fs::read_to_string(config.file_path.clone())
        .expect("Should have been able to read the file");

    for rangePair in contents.split(",") {
        let range: Vec<&str> = rangePair.split("-").collect();
        let min_range: i64 = range[0].parse().unwrap();
        let max_range: i64 = range[1].parse().unwrap();
        password += do_range_check(&config, min_range, max_range);
    }

    println!("Password is: {}", password);
}

fn do_range_check(config: &Config, min_range: i64, max_range: i64) -> i64 {
    let mut password_addition = 0;

    for i in min_range..=max_range {
        password_addition += do_password_strategy(i, config.strategy);
    }
    password_addition
}

fn do_password_strategy(value: i64, strategy: fn(i64) -> i64) -> i64 {
    strategy(value)
}

fn part_a_strategy(value: i64) -> i64 {
    let i_str = value.to_string();
    let i_str_size = i_str.chars().count();
    if !i_str_size.is_multiple_of(2) {
        return 0;
    }

    let half_size = i_str_size / 2;

    let first_half: i64 = i_str[0..half_size].parse().unwrap();
    let second_half: i64 = i_str[half_size..half_size * 2].parse().unwrap();

    if first_half == second_half {
        return value;
    }

    0
}

fn part_b_strategy(value: i64) -> i64 {
    1
}

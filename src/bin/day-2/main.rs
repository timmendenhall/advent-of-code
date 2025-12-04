use std::env;
use std::fs;
use std::process;

struct Config {
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let file_path = args[1].clone();

        Ok(Config { file_path })
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

    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    for rangePair in contents.split(",") {
        let range: Vec<&str> = rangePair.split("-").collect();
        let min_range: i64 = range[0].parse().unwrap();
        let max_range: i64 = range[1].parse().unwrap();
        password += do_range_check(min_range, max_range);
    }

    println!("Password is: {}", password);
}

fn do_range_check(min_range: i64, max_range: i64) -> i64 {
    let mut password_addition = 0;

    for i in min_range..=max_range {
        let i_str = i.to_string();
        let i_str_size = i_str.chars().count();
        if i_str_size % 2 != 0 {
            continue;
        }

        let half_size = i_str_size / 2;

        let first_half: i64 = i_str[0..half_size].parse().unwrap();
        let second_half: i64 = i_str[half_size..half_size * 2].parse().unwrap();

        if first_half == second_half {
            password_addition += i;
        }
    }
    password_addition
}

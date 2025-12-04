use std::env;
use std::fs;
use std::process;

struct Config {
    file_path: String,
    strategy: fn(&str) -> i64,
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
    let mut password = 0;

    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    for line in contents.lines() {
        password += do_password_strategy(line, config.strategy);
    }

    println!("Password is: {}", password);
}

fn do_password_strategy(bank_input: &str, strategy: fn(&str) -> i64) -> i64 {
    strategy(bank_input)
}

fn get_bank_value(bank_input: &str, num_batteries: usize) -> i64 {
    let bank_input_length = bank_input.chars().count();
    let mut bank_values: Vec<char> = Vec::new();
    let mut cursor = 0;

    for i in 0..num_batteries {
        let j: usize = num_batteries - i;
        let remaining_bank_input = &bank_input[cursor..=bank_input_length - j];

        let digit = remaining_bank_input.chars().max().unwrap();
        let digit_index = remaining_bank_input.find(digit).unwrap();

        cursor += digit_index + 1;

        bank_values.push(digit);
    }

    bank_values.iter().collect::<String>().parse().unwrap()
}

fn part_a_strategy(bank_input: &str) -> i64 {
    get_bank_value(bank_input, 2)
}

fn part_b_strategy(bank_input: &str) -> i64 {
    get_bank_value(bank_input, 12)
}

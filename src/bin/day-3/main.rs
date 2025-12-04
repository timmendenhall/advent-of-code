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

fn part_a_strategy(bank_input: &str) -> i64 {
    let bank_input_length = bank_input.chars().count();
    let first_digit = bank_input[..bank_input_length - 1].chars().max().unwrap();
    let first_digit_index = bank_input.find(first_digit).unwrap();
    let remaining_bank_input = &bank_input[first_digit_index + 1..];
    let second_digit = remaining_bank_input.chars().max().unwrap();

    let mut bank_value = String::new();
    bank_value.push(first_digit);
    bank_value.push(second_digit);

    bank_value.parse().unwrap()
}

fn part_b_strategy(bank_input: &str) -> i64 {
    const DIGITS_TO_OUTPUT: usize = 12;

    let bank_input_length = bank_input.chars().count();
    let mut bank_values: Vec<char> = Vec::new();
    let mut cursor = 0;

    for i in 0..DIGITS_TO_OUTPUT {
        let j: usize = DIGITS_TO_OUTPUT - i;
        let remaining_bank_input = &bank_input[cursor..=bank_input_length - j];

        let digit = remaining_bank_input.chars().max().unwrap();
        let digit_index = remaining_bank_input.find(digit).unwrap();

        cursor += digit_index + 1;

        bank_values.push(digit);
    }

    bank_values.iter().collect::<String>().parse().unwrap()
}

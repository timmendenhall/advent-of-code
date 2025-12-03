use std::env;
use std::fs;
use std::process;

mod tests;

const SAFE_VALUE_MAX: i32 = 100; // Exclusive, 100 = 0-99 range 

struct Config {
    file_path: String,
    should_count_all_0s: bool, // First part doesn't count all 0 clicks, the second part of day 1 does
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let file_path = args[1].clone();
        let should_count_all_0s = args[2].clone().parse().unwrap();

        Ok(Config {
            file_path,
            should_count_all_0s,
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
    let mut safe_value = 50;

    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");
    let mut lines_count = 0;

    for line in contents.lines() {
        let (new_safe_value, password_increment) =
            execute_line(line, safe_value, config.should_count_all_0s);
        lines_count += 1;
        password += password_increment;
        safe_value = new_safe_value
    }

    println!("Parsed lines: {}", lines_count);
    println!("Password is: {}", password);
}

fn execute_line(instruction: &str, current_value: i32, should_count_all_0s: bool) -> (i32, i32) {
    let mut result: i32 = 0;
    let password_increment_by;
    let mut rotate_by: i32 = 0;

    if instruction.starts_with("L") {
        let split: Vec<&str> = instruction.split('L').collect();
        rotate_by = split[1].parse().unwrap();
        result = rotate_left(current_value, rotate_by);
        rotate_by *= -1; // Inverting this value as it's added for the password_increment_by calc
    } else if instruction.starts_with("R") {
        let split: Vec<&str> = instruction.split('R').collect();
        rotate_by = split[1].parse().unwrap();
        result = rotate_right(current_value, rotate_by);
    }

    if should_count_all_0s {
        let mut rotations = 0;

        if current_value != 0 && current_value + rotate_by <= 0 {
            rotations += 1
        }

        rotations += (current_value + rotate_by).abs() / SAFE_VALUE_MAX;

        password_increment_by = rotations;
    } else {
        password_increment_by = if result == 0 { 1 } else { 0 };
    }

    (result, password_increment_by)
}

fn rotate_left(safe_value: i32, rotate_by: i32) -> i32 {
    let ret = safe_value - rotate_by;
    wrap_safe_value(ret)
}

fn rotate_right(safe_value: i32, rotate_by: i32) -> i32 {
    let ret = safe_value + rotate_by;
    wrap_safe_value(ret)
}

fn wrap_safe_value(value: i32) -> i32 {
    if value < 0 {
        let new_val = value.abs() % SAFE_VALUE_MAX;
        return (SAFE_VALUE_MAX - new_val) % SAFE_VALUE_MAX;
    }

    value % SAFE_VALUE_MAX
}

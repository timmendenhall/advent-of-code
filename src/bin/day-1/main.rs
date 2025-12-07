use advent_of_code::config::Config;
use std::env;
use std::fs;
use std::process;

mod tests;

const SAFE_VALUE_MAX: i32 = 100; // Exclusive, 100 = 0-99 range 

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

    for line in contents.lines() {
        let (new_safe_value, password_increment) =
            execute_line(line, safe_value, config.strategy == "part-a");
        password += password_increment;
        safe_value = new_safe_value
    }

    println!("Password is: {}", password);
}

fn execute_line(instruction: &str, current_value: i32, is_plan_a: bool) -> (i32, i32) {
    let mut result: i32 = 0;
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

    let password_increment_by = if is_plan_a {
        part_a_strategy(current_value, rotate_by)
    } else {
        part_b_strategy(result)
    };

    (result, password_increment_by)
}

fn part_a_strategy(current_value: i32, rotate_by: i32) -> i32 {
    let mut rotations = 0;

    if current_value != 0 && current_value + rotate_by <= 0 {
        rotations += 1
    }

    rotations += (current_value + rotate_by).abs() / SAFE_VALUE_MAX;

    rotations
}

fn part_b_strategy(result: i32) -> i32 {
    if result == 0 { 1 } else { 0 }
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

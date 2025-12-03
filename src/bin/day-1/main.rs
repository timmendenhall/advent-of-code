use std::env;
use std::fs;
use std::process;

const SAFE_VALUE_MAX: i32 = 100; // Exclusive, 100 = 0-99 range 

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
    let mut safe_value = 50;

    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");
    let mut lines_count = 0;

    for line in contents.lines() {
        safe_value = execute_line(line, safe_value);
        lines_count += 1;

        if safe_value == 0 {
            password += 1;
        }
    }

    println!("Parsed lines: {}", lines_count);
    println!("Password is: {}", password);
}

fn execute_line(instruction: &str, current_value: i32) -> i32 {
    if instruction.starts_with("L") {
        let split: Vec<&str> = instruction.split('L').collect();
        return rotate_left(current_value, split[1].parse().unwrap());
    } else if instruction.starts_with("R") {
        let split: Vec<&str> = instruction.split('R').collect();
        return rotate_right(current_value, split[1].parse().unwrap());
    }

    i32::MAX // Failed to find L/R
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_right() {
        let result = rotate_right(50, 2);
        assert_eq!(result, 52);

        // Wraps correctly
        let result = rotate_right(0, SAFE_VALUE_MAX);
        assert_eq!(result, 0);

        let result = rotate_right(SAFE_VALUE_MAX - 1, 1);
        assert_eq!(result, 0);

        let result = rotate_right(SAFE_VALUE_MAX, 5 + (5 * SAFE_VALUE_MAX));
        assert_eq!(result, 5);
    }

    #[test]
    fn test_rotate_left() {
        // Wraps correctly
        let result = rotate_left(0, 1);
        assert_eq!(result, SAFE_VALUE_MAX - 1);

        let result = rotate_left(0, SAFE_VALUE_MAX);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_execute_line() {
        let result = execute_line("L480", 50);
        assert_eq!(result, 70);

        let result = execute_line("L48", 0);
        assert_eq!(result, 52);

        let result = execute_line("R60", 50);
        assert_eq!(result, 10);
    }
}

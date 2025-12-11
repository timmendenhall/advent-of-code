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

fn calculate_math_set(math_problems: Vec<RefCell<Vec<String>>>) -> i64 {
    let mut total = 0;

    for set in math_problems {
        let mut parsed: Vec<i64> = Vec::new();

        for val in set.borrow().iter() {
            match val.trim() {
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
    let mut math_problems: Vec<RefCell<Vec<String>>> = Vec::new();

    for line in input.lines() {
        let split: Vec<&str> = line.split_whitespace().collect();
        for (x, cell) in split.iter().enumerate() {
            let cell_str = String::from(*cell);
            if let Some(existing_set) = math_problems.get(x) {
                existing_set.borrow_mut().push(cell_str);
            } else {
                let new_set: Vec<String> = vec![cell_str];
                math_problems.push(RefCell::from(new_set));
            }
        }
    }

    calculate_math_set(math_problems)
}

fn part_b_strategy(input: String) -> i64 {
    // The substr values that'll get converted to values based on x pos
    let mut math_problems_string: Vec<RefCell<Vec<String>>> = Vec::new();

    // Determine col break points
    let mut col_break_indices = Vec::new();
    let line: &str = input.lines().next().unwrap_or("");
    let mut cursor = 0;

    while let Some(a) = line[cursor..].find(' ') {
        if is_col_divider(&input, a + cursor) {
            col_break_indices.push(a + cursor);
        }
        cursor += a + 1;
    }

    // Builds the string from raw input, to be processed via alien math
    let mut start_x = 0;
    for end_x in col_break_indices {
        math_problems_string.push(build_math_problem_string(&input, start_x, end_x));
        start_x = end_x + 1;
    }
    // Need to add the last col
    let end_x = line.len();
    math_problems_string.push(build_math_problem_string(&input, start_x, end_x));

    // plan B the values - alien math
    let mut math_problems: Vec<RefCell<Vec<String>>> = Vec::new();
    for problem_set in math_problems_string {
        math_problems.push(build_math_problem(&problem_set));
    }

    // calc should work with now converted values
    calculate_math_set(math_problems)
}

fn build_math_problem(problem_set: &RefCell<Vec<String>>) -> RefCell<Vec<String>> {
    let mut num_strings: Vec<String> = Vec::new();

    for line in problem_set.borrow().iter().rev().skip(1).rev() {
        for (x, cell) in line.chars().enumerate() {
            let cell_str = String::from(cell);
            if let Some(existing_set) = num_strings.get_mut(x) {
                existing_set.push_str(&cell_str);
            } else {
                num_strings.push(cell_str);
            }
        }
    }

    let operation_string = String::from(problem_set.borrow().iter().last().unwrap());
    num_strings.push(operation_string);

    RefCell::from(num_strings)
}

fn build_math_problem_string(input: &str, start: usize, end: usize) -> RefCell<Vec<String>> {
    let mut ret = Vec::new();

    for line in input.lines() {
        let s = if line.len() >= end {
            line[start..end].to_string()
        } else {
            line[start..].to_string()
        };
        ret.push(s);
    }

    RefCell::from(ret)
}

fn is_col_divider(input: &str, x: usize) -> bool {
    for line in input.lines() {
        if line.chars().nth(x).unwrap() != ' ' {
            return false;
        }
    }

    true
}

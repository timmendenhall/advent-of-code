use std::env;
use std::fs;
use std::process;

struct Config {
    file_path: String,
    strategy: fn(Vec<(i64, i64)>, Vec<i64>) -> i64,
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

    let mut fresh_ingredients_string = Vec::new();
    let mut available_ingredients_string = Vec::new();
    let mut is_adding_fresh_ingredients = true;

    for line in contents.lines() {
        if line.trim().is_empty() {
            is_adding_fresh_ingredients = false;
            continue;
        }
        if is_adding_fresh_ingredients {
            fresh_ingredients_string.push(line);
        } else {
            available_ingredients_string.push(line);
        }
    }

    let fresh_ingredients = build_fresh_ingredients(&fresh_ingredients_string);
    let available_ingredients = build_available_ingredients(&available_ingredients_string);

    let password = (config.strategy)(fresh_ingredients, available_ingredients);

    println!("Password is: {}", password);
}

fn build_fresh_ingredients(inventory_ranges: &Vec<&str>) -> Vec<(i64, i64)> {
    let mut ret_vec = Vec::new();

    for line in inventory_ranges {
        let split: Vec<&str> = line.split('-').collect();
        let start: i64 = split[0].parse().unwrap();
        let stop: i64 = split[1].parse().unwrap();

        ret_vec.push((start, stop))
    }

    ret_vec
}

fn build_available_ingredients(available_ingredients: &Vec<&str>) -> Vec<i64> {
    let mut ret_vec: Vec<i64> = Vec::new();

    for line in available_ingredients {
        ret_vec.push(line.parse().unwrap());
    }

    ret_vec
}

fn part_a_strategy(
    fresh_ingredient_id_ranges: Vec<(i64, i64)>,
    available_ingredient_ids: Vec<i64>,
) -> i64 {
    let mut password_addition = 0;

    for i in available_ingredient_ids {
        if is_id_in_range(i, &fresh_ingredient_id_ranges) {
            password_addition += 1;
        }
    }

    password_addition
}

fn is_id_in_range(id: i64, fresh_ingredient_id_ranges: &Vec<(i64, i64)>) -> bool {
    for (start, end) in fresh_ingredient_id_ranges {
        if id >= *start && id <= *end {
            return true;
        }
    }

    false
}

fn part_b_strategy(
    fresh_ingredient_id_ranges: Vec<(i64, i64)>,
    _available_ingredient_ids: Vec<i64>,
) -> i64 {
    let mut final_ranges: Vec<(i64, i64)> = Vec::new();
    // let range_copy: Vec<(i64, i64)> = fresh_ingredient_id_ranges.clone();

    // First pass, get the total # of ids by calculating the difference of the ranges

    // let mut overlapping_ids = 0;
    // Second pass, check ranges for collisions and keep a count of how many should be reduced
    // Determine which range has the lower start value (range A) other range is range B
    // No overlap start if A.end < B.start
    // If A.end is in Range B overlap += A.end - B.start | 18 - 16 = +2 overlap
    // If A.start is in Range B overlap +=
    // Example:
    // A - 12-18
    // B - 16-20
    //
    // For each range (A), if in range B (all other ranges), then mutate A
    //
    for (start_a, end_a) in &fresh_ingredient_id_ranges {
        let mut new_start = *start_a;
        let mut new_end = *end_a;

        for (start_b, end_b) in &fresh_ingredient_id_ranges {
            // Don't check against itself
            if *start_a == *start_b && *end_a == *end_b {
                continue;
            }

            // A end is within B, move A end to B start
            if end_a > start_b && start_a < start_b {
                new_end = *start_b;
            }

            // A start is within B AND A end is > B end, move A start to B end
            if start_a >= start_b && start_a <= end_b && end_a > end_b {
                new_start = *end_b;
            }
            println!("!! A: {}-{} | B: {}-{}", *start_a, *end_a, *start_b, *end_b);
        }
        final_ranges.push((new_start, new_end));
    }

    let mut total_ids = 0;

    for (start, end) in &final_ranges {
        println!("Range to add: {}-{}", *start, *end);
        total_ids += *end - *start + 1;
    }

    total_ids
}

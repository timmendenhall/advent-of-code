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
    available_ingredient_ids: Vec<i64>,
) -> i64 {
    123
}

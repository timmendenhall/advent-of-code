use advent_of_code::config::Config;
use std::env;
use std::fs;
use std::process;

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

    let password = match config.strategy.as_str() {
        "part-a" => part_a_strategy(fresh_ingredients, available_ingredients),
        "part-b" => part_b_strategy(fresh_ingredients, available_ingredients),
        _ => part_a_strategy(fresh_ingredients, available_ingredients),
    };

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

fn is_range_collision(range_a: (i64, i64), range_b: (i64, i64)) -> bool {
    (range_a.0 >= range_b.0 && range_a.0 <= range_b.1)
        || (range_a.1 >= range_b.0 && range_a.1 <= range_b.1)
}

fn get_colliding_range_indices(all_ranges: &Vec<(i64, i64)>) -> Vec<(usize, usize)> {
    let mut colliding_range_indices: Vec<(usize, usize)> = Vec::new();
    let mut i_a = 0;
    let mut i_b = 0;

    // First pass, get ids (indices) of all ranges that collide and add the collision pairs to a vector
    for range_a in all_ranges {
        i_b = 0;
        for range_b in all_ranges {
            // Don't check against itself
            if i_a == i_b {
                i_b += 1;
                continue;
            }
            if is_range_collision(*range_a, *range_b) {
                colliding_range_indices.push((i_a, i_b));
            }

            i_b += 1;
        }

        i_a += 1;
    }

    colliding_range_indices
}

fn part_b_strategy(
    fresh_ingredient_id_ranges: Vec<(i64, i64)>,
    _available_ingredient_ids: Vec<i64>,
) -> i64 {
    let mut final_ranges: Vec<(i64, i64)> = fresh_ingredient_id_ranges.clone();

    loop {
        let a = get_colliding_range_indices(&final_ranges);
        if a.is_empty() {
            break;
        }

        let ranges_copy = final_ranges.clone();
        let mut indices_used = Vec::new();
        final_ranges.clear();

        // loop through all collision pairs and merge to a new range, add to final range vec and remove from possible IDs
        for (index_a, index_b) in a {
            if indices_used.contains(&index_a) || indices_used.contains(&index_b) {
                continue;
            }
            let range_a = ranges_copy.get(index_a).unwrap();
            let range_b = ranges_copy.get(index_b).unwrap();

            let new_start = if range_a.0 > range_b.0 {
                range_b.0
            } else {
                range_a.0
            };

            let new_end = if range_a.1 > range_b.1 {
                range_a.1
            } else {
                range_b.1
            };
            indices_used.push(index_a);
            indices_used.push(index_b);
            final_ranges.push((new_start, new_end));
        }

        for (i, range) in ranges_copy.iter().enumerate() {
            if indices_used.contains(&i) {
                continue;
            }
            final_ranges.push(*range);
        }
    }

    let mut total_ids = 0;

    for (start, end) in &final_ranges {
        total_ids += *end - *start + 1;
    }

    total_ids
}

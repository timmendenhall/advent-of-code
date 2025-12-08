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

    let mut paper_array = build_paper_array(contents);
    let password = match config.strategy.as_str() {
        "part-a" => part_a_strategy(&mut paper_array),
        "part-b" => part_b_strategy(&mut paper_array),
        _ => part_a_strategy(&mut paper_array),
    };

    println!("Password is: {}", password);
}

fn build_paper_array(inventory: String) -> Vec<Vec<bool>> {
    let mut ret_vec: Vec<Vec<bool>> = Vec::new();

    for line in inventory.lines() {
        // println!("{}", line);
        let mut line_vec: Vec<bool> = Vec::new();
        for inventory_value in line.chars() {
            line_vec.push(inventory_value == '@');
        }
        ret_vec.push(line_vec);
    }

    ret_vec
}

fn is_paper_accessible_to_forklift(paper_array: &[Vec<bool>], x: usize, y: usize) -> bool {
    if !is_paper(paper_array, x, y) {
        return false;
    }

    let adjacent_rolls = get_adjacent_rolls(paper_array, x, y);
    adjacent_rolls < 4
}

fn is_paper(paper_array: &[Vec<bool>], x: usize, y: usize) -> bool {
    let row = paper_array.get(y).unwrap();
    let col = row.get(x).unwrap();
    *col
}

fn get_adjacent_rolls(paper_array: &[Vec<bool>], x: usize, y: usize) -> usize {
    let row_length = paper_array.len();
    let col_length = paper_array.first().unwrap().len();

    let x_min = if x > 0 { x - 1 } else { 0 };
    let y_min = if y > 0 { y - 1 } else { 0 };

    let x_max = if x < col_length - 1 {
        x + 1
    } else {
        col_length - 1
    };
    let y_max = if y < row_length - 1 {
        y + 1
    } else {
        row_length - 1
    };

    let mut adjacent_rolls = 0;

    for y_iter in y_min..=y_max {
        for x_iter in x_min..=x_max {
            if (y == y_iter && x == x_iter) || !is_paper(paper_array, x_iter, y_iter) {
                continue;
            }
            adjacent_rolls += 1;
        }
    }

    adjacent_rolls
}

fn remove_all_possible_paper(paper_array: &mut [Vec<bool>]) -> i64 {
    // let mut paper_removed = 0;
    let mut coords_to_remove = Vec::new();

    for y in 0..paper_array.len() {
        for x in 0..paper_array[y].len() {
            if is_paper_accessible_to_forklift(paper_array, x, y) {
                coords_to_remove.push((x, y));
            }
        }
    }

    for (x, y) in coords_to_remove.iter() {
        remove_paper_at(paper_array, *x, *y);
    }

    coords_to_remove.len() as i64
}

fn remove_paper_at(paper_array: &mut [Vec<bool>], x: usize, y: usize) {
    paper_array[y][x] = false;
}

fn part_a_strategy(paper_array: &mut [Vec<bool>]) -> i64 {
    remove_all_possible_paper(paper_array)
}

fn part_b_strategy(paper_array: &mut [Vec<bool>]) -> i64 {
    let mut total_paper_removed = 0;
    let mut paper_removed = -1;

    while paper_removed != 0 {
        paper_removed = remove_all_possible_paper(paper_array);
        total_paper_removed += paper_removed;
    }

    total_paper_removed
}

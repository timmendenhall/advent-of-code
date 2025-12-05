use std::env;
use std::fs;
use std::process;

struct Config {
    file_path: String,
    strategy: fn(Vec<Vec<bool>>) -> i64,
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

    let paper_array = build_paper_array(contents);
    let password = (config.strategy)(paper_array);

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

fn part_a_strategy(paper_array: Vec<Vec<bool>>) -> i64 {
    let mut password = 0;

    for (y, row) in paper_array.iter().enumerate() {
        for (x, _col) in row.iter().enumerate() {
            if is_paper_accessible_to_forklift(&paper_array, x, y) {
                password += 1
            }
        }
    }

    password
}

fn is_paper_accessible_to_forklift(paper_array: &Vec<Vec<bool>>, x: usize, y: usize) -> bool {
    if !is_paper(paper_array, x, y) {
        return false;
    }

    let adjacent_rolls = get_adjacent_rolls(paper_array, x, y);
    adjacent_rolls < 4
}

fn is_paper(paper_array: &Vec<Vec<bool>>, x: usize, y: usize) -> bool {
    let row = paper_array.get(y).unwrap();
    let col = row.get(x).unwrap();
    *col
}

fn get_adjacent_rolls(paper_array: &Vec<Vec<bool>>, x: usize, y: usize) -> usize {
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

fn part_b_strategy(_paper_array: Vec<Vec<bool>>) -> i64 {
    0
}

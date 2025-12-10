use advent_of_code::config::Config;
use array2d::{Array2D, Error};
use std::env;
use std::fs;
use std::process;

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, PartialEq)]
enum ManifoldStatus {
    Empty,
    Start,
    Splitter,
    Beam,
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
    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    let tachyon_manifold_diagram = build_manifold(contents).unwrap();

    let password = match config.strategy.as_str() {
        "part-a" => process_manifold_a(tachyon_manifold_diagram),
        "part-b" => process_manifold_b(tachyon_manifold_diagram),
        _ => process_manifold_a(tachyon_manifold_diagram),
    };

    println!("Password is: {}", password);
}

fn process_manifold_a(manifold: Array2D<ManifoldStatus>) -> i64 {
    let mut split_count = 0;

    // for each row, queue up what the next row will contain for a "next row state"
    // compare diagram with next row state by looping over next row state chars() --
    // if beam in next state will hit current state splitter: split
    // if beam hits nothing, new next state has beam continue down
    let mut state = vec![ManifoldStatus::Empty; manifold.row_len()];

    for row in manifold.rows_iter() {
        let row_clone = row.clone();

        for (x, element) in row.enumerate() {
            match element {
                ManifoldStatus::Start => {
                    state[x] = ManifoldStatus::Beam;
                }
                ManifoldStatus::Splitter => {
                    if state[x] == ManifoldStatus::Beam {
                        split_count += 1;
                        split_beam(&mut state, x);
                    }
                }
                _ => {}
            }
        }

        let mut beam_count = 0;
        for (x, element) in row_clone.enumerate() {
            if state[x] == ManifoldStatus::Beam {
                beam_count += 1;
            }
            println!(
                "x: {:#?} | element: {:#?} | count: {}",
                x, element, beam_count
            );
        }
    }

    split_count
}

fn process_manifold_b(manifold: Array2D<ManifoldStatus>) -> i64 {
    let mut state = vec![ManifoldStatus::Empty; manifold.row_len()];

    // The 2D Array state of paths, each value is the number of paths that can reach that cell location
    let mut paths = Array2D::filled_with(0, manifold.num_rows(), manifold.num_columns());

    for (y, row) in manifold.rows_iter().enumerate() {
        // let row_copy = row.clone();

        for (x, element) in row.enumerate() {
            match element {
                ManifoldStatus::Start => {
                    state[x] = ManifoldStatus::Beam;
                    paths[(y, x)] = 1;
                }
                ManifoldStatus::Splitter => {
                    if state[x] == ManifoldStatus::Beam {
                        split_beam(&mut state, x);
                        let current_paths = if y > 0 { paths[(y - 1, x)] } else { 1 };
                        paths[(y, x - 1)] += current_paths;
                        paths[(y, x + 1)] += current_paths;
                    }
                }
                ManifoldStatus::Empty => {
                    if state[x] == ManifoldStatus::Beam {
                        let current_paths = if y > 0 { paths[(y - 1, x)] } else { 1 };
                        paths[(y, x)] += current_paths;
                    }
                }
                _ => {}
            }
        }
    }

    // Debug printing
    for row in paths.rows_iter() {
        for element in row {
            print!("{} ", element);
        }
        println!();
    }

    let mut sum = 0;
    if let Some(row) = paths.rows_iter().last() {
        for element in row {
            sum += element;
        }
        println!();
    }

    sum
}

fn split_beam(state: &mut [ManifoldStatus], split_at_x: usize) {
    state[split_at_x] = ManifoldStatus::Empty;
    let current_length = state.len();

    if split_at_x > 0 {
        state[split_at_x - 1] = ManifoldStatus::Beam;
    }

    if split_at_x < current_length {
        state[split_at_x + 1] = ManifoldStatus::Beam;
    }
}

fn build_manifold(input: String) -> Result<Array2D<ManifoldStatus>, Error> {
    let mut rows: Vec<Vec<ManifoldStatus>> = Vec::new();

    for line in input.lines() {
        let mut row: Vec<ManifoldStatus> = Vec::new();

        for char_to_check in line.chars() {
            match char_to_check {
                '.' => row.push(ManifoldStatus::Empty),
                'S' => row.push(ManifoldStatus::Start),
                '^' => row.push(ManifoldStatus::Splitter),
                _ => row.push(ManifoldStatus::Empty),
            }
        }

        rows.push(row);
    }

    let from_rows = Array2D::from_rows(&rows)?;
    Ok(from_rows)
}

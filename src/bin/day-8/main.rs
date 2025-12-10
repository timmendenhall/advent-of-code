use advent_of_code::config::Config;
use std::env;
use std::fs;
use std::process;

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, PartialEq)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    fn from(string: &str) -> Self {
        let split: Vec<&str> = string.split(',').collect();
        let x = split[0].parse().unwrap();
        let y = split[1].parse().unwrap();
        let z = split[2].parse().unwrap();
        Self { x, y, z }
    }
    fn distance_to(self: &Self, vec: &Vec3) -> f32 {
        let x = (self.x - vec.x).powi(2);
        let y = (self.y - vec.y).powi(2);
        let z = (self.z - vec.z).powi(2);

        (x + y + z).sqrt()
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
    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    let password = match config.strategy.as_str() {
        "part-a" => do_part_a(contents),
        "part-b" => do_part_b(contents),
        _ => do_part_a(contents),
    };

    println!("Password is: {}", password);
}

fn do_part_a(contents: String) -> i64 {
    let all_junction_boxes = build_junction_boxes(contents);
    let mut available_junction_boxes = all_junction_boxes.clone();
    let mut circuits: Vec<Vec<Vec3>> = Vec::new();

    for junction_box in all_junction_boxes {
        for checking_box in available_junction_boxes.iter() {
            let distance = junction_box.distance_to(checking_box);
            println!(
                "Checking {:#?} -> {:#?} | distance = {}",
                junction_box, checking_box, distance
            );
        }
    }

    1
}

fn do_part_b(contents: String) -> i64 {
    2
}

fn build_junction_boxes(contents: String) -> Vec<Vec3> {
    let mut junction_boxes = Vec::new();

    for line in contents.lines() {
        let junction_box = Vec3::from(line);
        junction_boxes.push(junction_box);
    }

    junction_boxes
}

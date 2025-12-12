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
    fn from(string: &str) -> Self {
        let split: Vec<&str> = string.split(',').collect();
        let x = split[0].parse().unwrap();
        let y = split[1].parse().unwrap();
        let z = split[2].parse().unwrap();
        Self { x, y, z }
    }

    fn distance_to(&self, vec: &Vec3) -> f32 {
        let x = (self.x - vec.x).powi(2);
        let y = (self.y - vec.y).powi(2);
        let z = (self.z - vec.z).powi(2);

        (x + y + z).sqrt()
    }
}

#[derive(Clone)]
struct Circuit {
    junction_boxes: Vec<Vec3>,
}

impl Circuit {
    fn new() -> Self {
        Self {
            junction_boxes: Vec::new(),
        }
    }

    fn add_junction(&mut self, junction_box: Vec3) {
        self.junction_boxes.push(junction_box);
    }

    fn has_junction(&self, junction_box: &Vec3) -> bool {
        self.junction_boxes.contains(junction_box)
    }

    fn len(&self) -> usize {
        self.junction_boxes.len()
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
    let available_junction_boxes = all_junction_boxes.clone();
    let mut closest_pairs = Vec::new();

    // Get all closest boxes first
    for junction_box in all_junction_boxes.iter() {
        if let Some(closest_box) = available_junction_boxes
            .iter()
            .filter(|jbox| *jbox != junction_box)
            .min_by(|a, b| {
                let da = junction_box.distance_to(a);
                let db = junction_box.distance_to(b);
                da.partial_cmp(&db).unwrap()
            })
        {
            closest_pairs.push((
                junction_box,
                closest_box,
                junction_box.distance_to(closest_box),
            ));
        }
    }

    // Sort by distance ascending
    closest_pairs
        .sort_by(|(_a1, _a2, dist_a), (_b1, _b2, dist_b)| dist_a.partial_cmp(dist_b).unwrap());

    // then calculate the results

    let mut circuits: Vec<Circuit> = Vec::new();
    circuits.sort_by_key(|c| std::cmp::Reverse(c.len()));

    for c in circuits.iter().take(5) {
        println!("SORTED {}", c.len());
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

fn add_boxes_to_circuits(circuits: &mut Vec<Circuit>, box_a: &Vec3, box_b: &Vec3) {
    let a_idx = circuits.iter().position(|c| c.has_junction(box_a));
    let b_idx = circuits.iter().position(|c| c.has_junction(box_b));

    match (a_idx, b_idx) {
        // A in circuit, B not in any
        (Some(i), None) => {
            circuits[i].add_junction(box_b.clone());
        }

        // B in circuit, A not in any
        (None, Some(i)) => {
            circuits[i].add_junction(box_a.clone());
        }

        // Neither in any circuit -> make new
        (None, None) => {
            let mut c = Circuit::new();
            c.add_junction(box_a.clone());
            c.add_junction(box_b.clone());
            circuits.push(c);
        }

        // Both already in the same circuit -> nothing to do
        (Some(_), Some(_)) => { /* do nothing */ }
    }
}

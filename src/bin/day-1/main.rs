use std::env;
use std::fs;

const SAFE_VALUE_MAX: i32 = 100; // Exclusive, 100 = 0-99 range 

fn main() {
    println!("Hello, world!");
    do_puzzle()
}

fn do_puzzle() {
    let _password = 0;
    let _safe_value = 50;

    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    println!("{}", contents);
}

fn execute_line(instruction: &str, current_value: i32) -> i32 {
    let rotate_by = 12; // TODO: <--
    if instruction.starts_with("L") {
        return rotate_left(current_value, rotate_by);
    }
    return rotate_right(current_value, rotate_by);
}

fn rotate_left(safe_value: i32, rotate_by: i32) -> i32 {
    let mut ret = safe_value - rotate_by;
    ret % SAFE_VALUE_MAX
}

fn rotate_right(safe_value: i32, rotate_by: i32) -> i32 {
    let ret = safe_value + rotate_by;
    ret % SAFE_VALUE_MAX
}

#[test]
fn test_rotate_right() {
    let result = rotate_right(50, 2);
    assert_eq!(result, 52);

    // Wraps over max value
    let result = rotate_right(SAFE_VALUE_MAX, 2);
    assert_eq!(result, 1);
}

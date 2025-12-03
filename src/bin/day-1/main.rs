use std::env;
use std::fs;

fn main() {
    println!("Hello, world!");
    do_puzzle()
}

fn do_puzzle() {
    let _password = 0;
    let _safe_value = 50;
}

fn rotate_right(safe_value: &mut i32, rotate_by: i32) {
    *safe_value += rotate_by;
}

#[test]
fn test_rotate_right() {
    let mut safe_value = 50;

    rotate_right(&mut safe_value, 2);

    assert_eq!(safe_value, 52);
}

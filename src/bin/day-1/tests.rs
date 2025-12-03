use crate::*;

#[test]
fn test_rotate_right() {
    let result = rotate_right(50, 2);
    assert_eq!(result, 52);

    // Wraps correctly
    let result = rotate_right(0, SAFE_VALUE_MAX);
    assert_eq!(result, 0);

    let result = rotate_right(SAFE_VALUE_MAX - 1, 1);
    assert_eq!(result, 0);

    let result = rotate_right(SAFE_VALUE_MAX, 5 + (5 * SAFE_VALUE_MAX));
    assert_eq!(result, 5);
}

#[test]
fn test_rotate_left() {
    // Wraps correctly
    let result = rotate_left(0, 1);
    assert_eq!(result, SAFE_VALUE_MAX - 1);

    let result = rotate_left(0, SAFE_VALUE_MAX);
    assert_eq!(result, 0);
}

#[test]
fn test_execute_line() {
    let (result, _password_increase_by) = execute_line("L480", 50, false);
    assert_eq!(result, 70);

    let (result, _password_increase_by) = execute_line("L48", 0, false);
    assert_eq!(result, 52);

    let (result, _password_increase_by) = execute_line("R60", 50, false);
    assert_eq!(result, 10);

    let (result, password_increase_by) = execute_line("L68", 50, true);
    assert_eq!(result, 82);
    assert_eq!(password_increase_by, 1);

    let (result, password_increase_by) = execute_line("L30", 82, true);
    assert_eq!(result, 52);
    assert_eq!(password_increase_by, 0);

    let (result, password_increase_by) = execute_line("R48", 52, true);
    assert_eq!(result, 0);
    assert_eq!(password_increase_by, 1);

    let (result, password_increase_by) = execute_line("R60", 95, true);
    assert_eq!(result, 55);
    assert_eq!(password_increase_by, 1);

    let (result, password_increase_by) = execute_line("L55", 55, true);
    assert_eq!(result, 0);
    assert_eq!(password_increase_by, 1);

    let (result, password_increase_by) = execute_line("L99", 99, true);
    assert_eq!(result, 0);
    assert_eq!(password_increase_by, 1);

    let (result, password_increase_by) = execute_line("L82", 14, true);
    assert_eq!(result, 32);
    assert_eq!(password_increase_by, 1);

    let (result, password_increase_by) = execute_line("R548", 52, true);
    assert_eq!(result, 0);
    assert_eq!(password_increase_by, 6);

    let (result, password_increase_by) = execute_line("R102", 99, true);
    assert_eq!(result, 1);
    assert_eq!(password_increase_by, 2);

    let (result, password_increase_by) = execute_line("R502", 99, true);
    assert_eq!(result, 1);
    assert_eq!(password_increase_by, 6);

    let (result, password_increase_by) = execute_line("L55", 55, true);
    assert_eq!(result, 0);
    assert_eq!(password_increase_by, 1);
}

use crate::*;

#[test]
fn test_part_a_strategy() {
    let result = part_a_strategy("1000098");
    assert_eq!(result, 98);

    let result = part_a_strategy("987654321111111");
    assert_eq!(result, 98);

    let result = part_a_strategy("811111111111119");
    assert_eq!(result, 89);

    let result = part_a_strategy("234234234234278");
    assert_eq!(result, 78);

    let result = part_a_strategy("818181911112111");
    assert_eq!(result, 92);
}

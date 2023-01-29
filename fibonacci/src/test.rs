use super::*;

#[test]
fn zero_fibonacci() {
    let n = 0;
    let result = handle_edge_cases(n);
    assert_eq!(result, 0);
}

#[test]
fn first_fibonacci() {
    let n = 1;
    let result = handle_edge_cases(n);
    assert_eq!(result, 1);
}

#[test]
fn fifth_fibonacci() {
    let memo: [Integer; 2] = [Integer::from(0), Integer::from(1)];
    let n = 5;
    let result = handle_default_cases(n, memo);
    assert_eq!(result, 5);
}

#[test]
fn millionth_fibonacci() {
    let memo: [Integer; 2] = [Integer::from(0), Integer::from(1)];
    let n = 1000000;
    let result = handle_default_cases(n, memo);
    let result_as_string = &result.to_string_radix(10)[..10];
    assert_eq!(result_as_string, String::from("1953282128"));
}

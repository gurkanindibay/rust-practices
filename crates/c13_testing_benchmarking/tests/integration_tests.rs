use c13_testing_benchmarking::{divide, sanitize_input};

#[test]
fn test_end_to_end_workflow() {
    let clean = sanitize_input(" 42.0 ");
    assert_eq!(clean, "42.0");

    let val: f64 = clean.parse().unwrap();
    let divided = divide(val, 2.0).unwrap();
    assert_eq!(divided, 21.0);
}

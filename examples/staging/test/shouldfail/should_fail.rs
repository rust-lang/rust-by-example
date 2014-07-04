#[test]
#[should_fail]
fn out_of_bounds_test() {
    let xs = vec![0u, 1, 3, 5, 9, 11];
    // This index is out of bounds, causing a task failure.
    xs.get(7000);
}

#[test]
#[should_fail]
fn non_failing_test() {
    // No task failure here. This test will not pass.
    assert!(true);
}

#[test]
fn abs_positive_test() {
    let n = -20i;
    assert!(n.abs().is_positive());
    println!("abs_positive_test ran with n = {}", n);
}

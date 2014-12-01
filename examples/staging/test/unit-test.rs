// For .powi()
use std::num::Float;


fn distance(a: (f32, f32), b: (f32, f32)) -> f32 {
    (
        (b.val0().powi(2) - a.val0().powi(2)) +
        (b.val1().powi(2) - a.val1().powi(2))
    ).sqrt()
}

fn main() {
    println!("If you see this, the tests were not compiled nor ran!");
}

#[test]
fn distance_test() {
    assert!(distance((0f32, 0f32), (1f32, 1f32)) == (2f32).sqrt());
}

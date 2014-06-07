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
    assert!(distance((0.0, 0.0), (1.0, 1.0)) == (2 as f32).sqrt());
}
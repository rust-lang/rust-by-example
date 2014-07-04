// The main function is only executed in a normal build.
// A test build will execute the test runner instead.
fn main() {
    let squares = setup_test_subject(5);
    for square in squares.iter() {
        println!("{} is a square number", square);
    }
}

#[cfg(test)]
// TODO ^ Try removing this line to let main use the function too.
fn setup_test_subject(n: uint) -> Vec<f32> {
    Vec::from_fn(n, |i| (i * i) as f32)
}

#[test]
fn square_test() {
    let squares = setup_test_subject(20);
    for square in squares.iter() {
        assert_eq!(square.sqrt().floor(), square.sqrt());
    }
}

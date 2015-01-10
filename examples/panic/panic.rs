// Re-implementation of integer division (/)
fn division(dividend: isize, divisor: isize) -> isize {
    if divisor == 0 {
        // Division by zero triggers a task panic
        panic!("division by zero");
    } else {
        dividend / divisor
    }
}

// The `main` task
fn main() {
    // Heap allocated integer
    let _x = box 0is;

    // This operation will trigger a task failure
    division(3, 0);

    println!("This point won't be reached!");

    // `_x` should get destroyed at this point
}

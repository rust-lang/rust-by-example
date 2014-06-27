// An integer division that doesn't `fail!`
fn checked_division(dividend: int, divisor: int) -> Option<int> {
    if divisor == 0 {
        // Failure is represented as the `None` variant
        None
    } else {
        // Result is wrapped in a `Some` variant
        Some(dividend / divisor)
    }
}

// This function handles a division that may not succeed
fn try_division(dividend: int, divisor: int) {
    // `Option` values can be pattern matched, just like other enums
    match checked_division(dividend, divisor) {
        None => println!("{} / {} failed!", dividend, divisor),
        Some(quotient) => {
            println!("{} / {} = {}", dividend, divisor, quotient)
        },
    }
}

fn main() {
    try_division(4, 2);
    try_division(1, 0);

    // Binding `None` to a variable needs to be type annotated
    let none: Option<int> = None;
    let _equivalent_none = None::<int>;

    let optional_float = Some(0f32);

    // The `unwrap` method will extract the value wrapped in a `Some` variant,
    // or will `fail!` if called on a `None` variant
    println!("{} unwraps to {}", optional_float, optional_float.unwrap());
    println!("{} unwraps to {}", none, none.unwrap());
}

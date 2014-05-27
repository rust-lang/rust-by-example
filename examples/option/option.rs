// re-implementation of division (/)
fn division(dividend: int, divisor: int) -> int {
    if divisor == 0 {
        fail!("division by zero");
    } else {
        dividend / divisor
    }
}

// checked division
fn checked_division(dividend: int, divisor: int) -> Option<int> {
    if divisor == 0 {
        // failure represented as None
        None
    } else {
        // result wrapped in a Some tuple struct
        Some(dividend / divisor)
    }
}

fn try_division(dividend: int, divisor: int) {
    // Option values can be pattern matched
    match checked_division(dividend, divisor) {
        None => println!("{} / {} failed!", dividend, divisor),
        Some(quotient) => println!("{} / {} = {}",
                                   dividend, divisor, quotient),
    }
}

fn main() {
    try_division(4, 2);
    try_division(1, 0);

    // Binding None to a variable needs to be type annotated
    let none: Option<int> = None;

    let optional_float = Some(0.0);

    // The unwrap() method will extract the wrapped value from Some() and
    // will fail! if called on None
    let unwrapped_float = optional_float.unwrap();
    let runtime_failure = none.unwrap();
}

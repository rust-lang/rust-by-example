use std::num::ParseIntError;
use std::result;

// Define a generic alias for a `Result` with the error type `ParseIntError`.
type AliasedResult<T> = result::Result<T, ParseIntError>;

// Use the alias defined above to refer to our specific `Result` type.
fn double_number(number_str: &str) -> AliasedResult<i32> {
    number_str.parse::<i32>().map(|n| 2 * n)
}

// Here, the alias again allows us to save some space.
fn print(result: AliasedResult<i32>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(double_number("10"));
    print(double_number("t"));
}

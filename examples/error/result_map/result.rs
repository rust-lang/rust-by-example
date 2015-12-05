use std::num::ParseIntError;

// With the return type rewritten, we proceed to use pattern matching without
// `unwrap()` but it is tedious. Couldn't a combinator like in the `Option`
// example also be used here? Yes.
fn double_number(number_str: &str) -> Result<i32, ParseIntError> {
    match number_str.parse::<i32>() {
        Ok(n)  => Ok(2 * n),
        Err(e) => Err(e),
    }
}

// The exact same but written with `map()`. Modify if the value is valid,
// otherwise pass the error on.
fn double_number_map(number_str: &str) -> Result<i32, ParseIntError> {
    number_str.parse::<i32>().map(|n| 2 * n)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    // Still presents a reasonable answer.
    let twenty = double_number("10");
    print(twenty);

    // This is now much better than before with the messy `panic`.
    let tt = double_number_map("t");
    print(tt);
}

fn double_number(number_str: &str) -> i32 {
    // It might not always be possible to parse a string into the other type
    // so `parse()` returns a `Result` indicating possible failure. Let's
    // just try `unwrap()` to get the number out. Will it bite us?
    2 * number_str.parse::<i32>().unwrap()
}

fn main() {
    let twenty = double_number("10");
    println!("double is {}", twenty);

    let tt = double_number("t");
    println!("double is {}", tt);
}

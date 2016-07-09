fn double_number(number_str: &str) -> i32 {
    // Let's try using `unwrap()` to get the number out. Will it bite us?
    2 * number_str.parse::<i32>().unwrap()
}

fn main() {
    let twenty = double_number("10");
    println!("double is {}", twenty);

    let tt = double_number("t");
    println!("double is {}", tt);
}

// Our first attempt uses `unwrap` and unhelpfully panics.
fn double_first(vec: Vec<&str>) -> i32 {
    // Panics if the input vector is empty:
    let first = vec.first().unwrap();

    // Panics if the element doesn't parse to a number:
    2 * first.parse::<i32>().unwrap()
}

fn main() {
    let numbers = vec!["93", "18"];
    let strings = vec!["tofu", "cheese", "bell pepper"];
    let empty = vec![];

    println!("The first doubled is {}", double_first(numbers));

    // This line results in the first panic:
    println!("The first doubled is {}", double_first(strings));
    // ^ Comment this out to see the second panic.

    // This line results in a second panic:
    println!("The first doubled is {}", double_first(empty));
}

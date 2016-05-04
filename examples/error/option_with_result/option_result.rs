// Our first attempt uses `unwrap` and provides unhelpful errors.
fn double_first(vec: Vec<&str>) -> i32 {
    // Returns an error if the input vector is empty:
    let first = vec.first().unwrap();

    // Returns an error if the element doesn't parse to a number:
    2 * first.parse::<i32>().unwrap()
}

fn main() {
    let numbers = vec!["93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {}", double_first(numbers));
    
    // This line results in the first error:
    println!("The first doubled is {}", double_first(empty));
    // ^ Comment this out to see the second error.
    
    // This line results in a second error:
    println!("The first doubled is {}", double_first(strings));
}

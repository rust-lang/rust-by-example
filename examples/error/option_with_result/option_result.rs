// The first attempt conveniently uses `unwrap` with the aforementioned
// bad errors it results in.
fn double_first(vec: Vec<&str>) -> i32 {
    // What if the vector is empty?
    let first = vec.first().unwrap();

    // What if the element doesn't parse to a number?
    2 * first.parse::<i32>().unwrap()
}

fn main() {
    let numbers = vec!["93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {}", double_first(numbers));
    println!("The first doubled is {}", double_first(empty));
    // ^ Comment out this line to see the second error.
    println!("The first doubled is {}", double_first(strings));
}

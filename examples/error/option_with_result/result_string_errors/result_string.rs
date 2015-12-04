type Result<T> = std::result::Result<T, String>;

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
       // Convert the `Option` to a `Result` if there is a value; otherwise
       // use an `Err` containing this `String`.
       .ok_or("Please use a vector with at least one element.".to_owned())
       // `parse` returns a `Result<T, ParseIntError>`.
       .and_then(|s| s.parse::<i32>()
                      //  The return type is `Result<T, String>`. We need
                      //  to map only the errors `parse` yields to `String`.
                      .map_err(|e| e.to_string())
                      // Apply the double to the number inside.
                      .map(|i| 2 * i))
}

fn print(result: Result<i32>) {
    match result {
        Ok(n)  => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let numbers = vec!["93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}

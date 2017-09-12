# Early returns

In the previous example, we explicitly handled the errors using combinators. 
Another way to deal with this case analysis is to use a combination of 
`match` statements and *early returns*. 

That is, we can simply stop executing the function and return the error if 
one occurs. For some, this form of code can be easier to both read and 
write. Consider this version of the previous example, rewritten using early returns:

```rust,editable
// Use `String` as our error type
type Result<T> = std::result::Result<T, String>;

fn double_first(vec: Vec<&str>) -> Result<i32> {
   // Convert the `Option` to a `Result` if there is a value.
   // Otherwise, provide an `Err` containing this `String`.
    let first = match vec.first() {
        Some(first) => first,
        None => return Err("Please use a vector with at least one element.".to_owned())
    };

    // Double the number inside if `parse` works fine.
    // Otherwise, map any errors that `parse` yields to `String`.
    match first.parse::<i32>() {
        Ok(i) => Ok(2 * i),
        Err(e) => Err(e.to_string()),
    }
}

fn print(result: Result<i32>) {
    match result {
        Ok(n)  => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(empty));
    print(double_first(strings));
}
```

At this point, we've learned to explicitly handle errors using combinators 
and early returns. While we generally want to avoid panicking, explicitly 
handling all of our errors is cumbersome.

In the next section, we'll introduce `try!` for the cases where we simply 
need to `unwrap` without possibly inducing `panic`. 

The previous examples have always been very convenient; `Result`s interact 
with other `Result`s and `Option`s interact with other `Option`s. 

Sometimes an `Option` needs to interact with a `Result`, or a `Result<T, Error1>` 
needs to interact with a `Result<T, Error2>`. In those cases, we want to 
handle the *composition of distinct error types*.

In the following code, two instances of `unwrap` generate different error 
types. `Vec::first` returns an `Option`, while `parse::<i32>` returns a 
`Result<i32, ParseIntError>`:

```rust
fn double_first(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap(); // Generate error 1
    2 * first.parse::<i32>().unwrap() // Generate error 2
}

fn main() {
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {}", double_first(empty));
    // Error 1: the input vector is empty

    println!("The first doubled is {}", double_first(strings));
	// Error 2: the element doesn't parse to a number
}
```

Using our knowledge of combinators, we can rewrite the above to explicitly
handle errors. In doing so, we convert both `Option`s and `Result`s into 
`Result`s, and map them to the same error type (`String`):

{multiple_error_types.play}

### See Also:

[`Option::ok_or`][okor], [Result::map_err][maperr]

[okor]: https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or
[maperr]: https://doc.rust-lang.org/std/result/enum.Result.html#method.map_err

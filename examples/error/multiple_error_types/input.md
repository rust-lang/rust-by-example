The previous examples have always been very convenient; `Result`s interact
with other `Result`s and `Option`s interact with other `Option`s.

Sometimes an `Option` needs to interact with a `Result`, or a
`Result<T, Error1>` needs to interact with a `Result<T, Error2>`. In those
cases, we want to manage our different error types in a way that makes them
composable and easy to interact with.

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
handle errors. Since two different types of errors can occur, we need to
convert them to a common type such as a `String`.

To do so, we convert both the `Option` and `Result` into `Result`s, and
then map their errors to the same type:

{multiple_error_types.play}

In the next section, we'll see an alternate method of explicitly handling these errors.

### See Also:

[`Option::ok_or`][okor], [`Result::map_err`][maperr]

[okor]: https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or
[maperr]: https://doc.rust-lang.org/std/result/enum.Result.html#method.map_err

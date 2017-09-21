# Defining an error type

Rust allows us to define our own error types. In general, a "good" error type:

* Represents different errors with the same type
* Presents nice error messages to the user
* Is easy to compare with other types
    - Good: `Err(EmptyVec)`
    - Bad: `Err("Please use a vector with at least one element".to_owned())`
* Can hold information about the error
    - Good: `Err(BadChar(c, position))`
    - Bad: `Err("+ cannot be used here".to_owned())`

Note that a `String` (which we've been using up to this point) fulfills the 
first two criteria, but not the last two. This makes `String` errors verbose 
to create and difficult to react to. It should not be necessary to pollute 
logic heavy code with `String` formatting simply to display nicely.

```rust,editable
use std::num::ParseIntError;
use std::fmt;

type Result<T> = std::result::Result<T, DoubleError>;

#[derive(Debug)]
// Define our error types. These may be customized for our error handling cases. 
// Now we will be able to write our own errors, defer to an underlying error
// implementation, or do something in between.
enum DoubleError {
    // We don't require any extra info to detail this error.
    EmptyVec,
    // We will defer to the parse error implementation for their error.
    // Supplying extra info requires adding more data to the type.
    Parse(ParseIntError),
}

// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DoubleError::EmptyVec =>
                write!(f, "please use a vector with at least one element"),
            // This is a wrapper, so defer to the underlying types' implementation of `fmt`.
            DoubleError::Parse(ref e) => e.fmt(f),
        }
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
       // Change the error to our new type.
       .ok_or(DoubleError::EmptyVec)
       .and_then(|s| s.parse::<i32>()
            // Update to the new error type here also.
            .map_err(DoubleError::Parse)
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
```

### See also:

[`Result`][result] and [`io::Result`][io_result]

[result]: https://doc.rust-lang.org/std/result/enum.Result.html
[io_result]: https://doc.rust-lang.org/std/io/type.Result.html
[inplace]: /error/option_with_result/result_string_errors.html

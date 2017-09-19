# `Box`ing errors

By implementing `Display` and `From` for our error type, we enabled
almost all of the `std` library error handling tools. However, we missed 
something: the ability to easily `Box` our error type.

The `std` library automatically converts any type that implements the 
`Error` trait into the trait object `Box<Error>`, via `From`. To a 
library user, this conveniently allows the following:

```rust,ignore
fn foo(...) -> Result<T, Box<Error>> { ... }
```

A user may use any variety of external libraries which each provide their own error
types. In order to define a valid `Result<T, E>` type, the user has a few choices:

* define a new wrapper error type around the library's error types
* convert the error types to `String` or another intermediate choice
* `Box` the error types into `Box<Error>` via type erasure

"Boxing" the error type is a common choice. The drawback is that the 
underlying error type is only known at runtime and not 
[statically determined][dynamic_dispatch]. As mentioned above, all that 
needs to be done is to implement the `Error` trait:

```rust,ignore
trait Error: Debug + Display {
    fn description(&self) -> &str;
    fn cause(&self) -> Option<&Error>;
}
```

With this implementation, let's look at our most recent example. Note that 
it is just as valid with the error type of `Box<Error>` as it was before 
with `DoubleError`:

```rust,editable
use std::error;
use std::fmt;
use std::num::ParseIntError;

// Change the alias to `Box<error::Error>`.
type Result<T> = std::result::Result<T, Box<error::Error>>;

#[derive(Debug)]
enum DoubleError {
    EmptyVec,
    Parse(ParseIntError),
}

impl From<ParseIntError> for DoubleError {
    fn from(err: ParseIntError) -> DoubleError {
        DoubleError::Parse(err)
    }
}

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DoubleError::EmptyVec =>
                write!(f, "please use a vector with at least one element"),
            DoubleError::Parse(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for DoubleError {
    fn description(&self) -> &str {
        match *self {
            // A very short description of the error. Doesn't need to be the
            // same as `Display`.
            DoubleError::EmptyVec => "empty vectors not allowed",
            // This already impls `Error`, so defer to its own implementation.
            DoubleError::Parse(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            // No underlying cause so return `None`.
            DoubleError::EmptyVec => None,
            // The cause is the underlying implementation error type. Is implicitly
            // cast to the trait object `&error::Error`. This works because the
            // underlying type already implements the `Error` trait.
            DoubleError::Parse(ref e) => Some(e),
        }
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().ok_or(DoubleError::EmptyVec)?;
    let parsed = first.parse::<i32>()?;

    Ok(2 * parsed)
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

[Dynamic dispatch][dynamic_dispatch] and [`Error` trait][error]

[dynamic_dispatch]: https://doc.rust-lang.org/book/trait-objects.html#dynamic-dispatch
[error]: https://doc.rust-lang.org/std/error/trait.Error.html

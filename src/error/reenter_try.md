# Other uses of `try!`

Notice in the previous example that our immediate reaction to calling 
`parse` is to `map` the error from a library error into our new custom 
error type:

```rust
.and_then(|s| s.parse::<i32>())
    .map_err(DoubleError::Parse)
```

Since this is a simple and common operation, it would be convenient if it 
could be elided. Alas, because `and_then` is not sufficiently flexible, it 
cannot. However, we can instead use `try!`.

`try!` was previously explained as either `unwrap` or `return Err(err)`. 
This is only mostly true. It actually means `unwrap` or 
`return Err(From::from(err))`. Since `From::from` is a conversion utility 
between different types, this means that if you `try!` where the error is 
convertible to the return type, it will convert automatically.

Here, we rewrite the previous example using `try!`. As a result, the 
`map_err` will go away when `From::from` is implemented for our error type:

```rust,editable
use std::num::ParseIntError;
use std::fmt;

type Result<T> = std::result::Result<T, DoubleError>;

#[derive(Debug)]
enum DoubleError {
    EmptyVec,
    Parse(ParseIntError),
}

// Implement the conversion from `ParseIntError` to `DoubleError`. 
// This will be automatically called by `try!` if a `ParseIntError` 
// needs to be converted into a `DoubleError`.
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

// The same structure as before but rather than chain all `Results`
// and `Options` along, we `try!` to get the inner value out immediately.
fn double_first(vec: Vec<&str>) -> Result<i32> {
    // Still convert to `Result` by stating how to convert `None`.
    let first = try!(vec.first().ok_or(DoubleError::EmptyVec));
    let parsed = try!(first.parse::<i32>());

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

This is actually fairly clean now. Compared with the original `panic`, it 
is very similar to replacing the `unwrap` calls with `try!` except that the 
return types are `Result`. As a result, they must be destructured at the 
top level.

Note that you should not expect error handling of this sort to always 
replace `unwrap`. This type of error handling tripled our line count and 
cannot really be considered simple (even when heavily biased by the small 
code size).

Indeed, moving a 1000 line library from `unwrap` to more proper error 
handling might be feasible in an additional 100 lines of code. However, the 
necessary refactoring would most definitely not be trivial.

Many libraries might get away with only implementing `Display` and 
adding `From` on an as needed basis. However, more serious libraries will 
eventually need to meet higher expectations of error handling implementation.

### See also:

[`From::from`][from] and [`try!`][try]

[from]: https://doc.rust-lang.org/std/convert/trait.From.html
[try]: https://doc.rust-lang.org/std/macro.try!.html

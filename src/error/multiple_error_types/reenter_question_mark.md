# Other uses of `?`

Notice in the previous example that our immediate reaction to calling `parse` is
to `map` the error from a library error into a boxed error:

```rust,ignore
.and_then(|s| s.parse::<i32>())
    .map_err(|e| e.into())
```

Since this is a simple and common operation, it would be convenient if it could
be elided. Alas, because `and_then` is not sufficiently flexible, it cannot.
However, we can instead use `?`.

`?` was previously explained as either `unwrap` or `return Err(err)`. This is
only mostly true. It actually means `unwrap` or `return Err(From::from(err))`.
Since `From::from` is a conversion utility between different types, this means
that if you `?` where the error is convertible to the return type, it will
convert automatically.

Here, we rewrite the previous example using `?`. As a result, the `map_err` will
go away when `From::from` is implemented for our error type:

```rust,editable
use std::error;
use std::fmt;

// Change the alias to use `Box<dyn error::Error>`.
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec {}

// The same structure as before but rather than chain all `Results`
// and `Options` along, we `?` to get the inner value out immediately.
fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().ok_or(EmptyVec)?;
    let parsed = first.parse::<i32>()?;
    Ok(2 * parsed)
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
```

This is actually fairly clean now. Compared with the original `panic`, it is
very similar to replacing the `unwrap` calls with `?` except that the return
types are `Result`. As a result, they must be destructured at the top level.

### See also:

[`From::from`][from] and [`?`][q_mark]

[from]: https://doc.rust-lang.org/std/convert/trait.From.html
[q_mark]: https://doc.rust-lang.org/reference/expressions/operator-expr.html#the-question-mark-operator

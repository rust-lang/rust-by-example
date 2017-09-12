# `map` for `Result`

Panicking in the previous example gave us an unhelpful error message.
To avoid that, we need to be more specific about the return type. There, the 
regular element is of type `i32`.

To determine the `Err` type, we look to 
[`parse()`][parse], which is implemented with the [`FromStr`][from_str] trait for 
[`i32`][i32]. As a result, the `Err` type is specified as [`ParseIntError`][parse_int_error].

In the example below, the straightforward `match` statement leads to code 
that is overall more cumbersome. Luckily, the `map` method of `Option` is 
one of many combinators also implemented for `Result`. [`enum.Result`][result] 
contains a complete listing.

```rust,editable
use std::num::ParseIntError;

// With the return type rewritten, we use pattern matching without `unwrap()`.
fn double_number(number_str: &str) -> Result<i32, ParseIntError> {
    match number_str.parse::<i32>() {
        Ok(n)  => Ok(2 * n),
        Err(e) => Err(e),
    }
}

// As with `Option`, we can use combinators such as `map()`.
// This function is otherwise identical to the one above and reads:
// Modify n if the value is valid, otherwise pass on the error.
fn double_number_map(number_str: &str) -> Result<i32, ParseIntError> {
    number_str.parse::<i32>().map(|n| 2 * n)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    // This still presents a reasonable answer.
    let twenty = double_number("10");
    print(twenty);

    // The following now provides a much more helpful error message.
    let tt = double_number_map("t");
    print(tt);
}
```

[parse]: https://doc.rust-lang.org/std/primitive.str.html#method.parse
[from_str]: https://doc.rust-lang.org/std/str/trait.FromStr.html
[i32]: https://doc.rust-lang.org/std/primitive.i32.html
[parse_int_error]: https://doc.rust-lang.org/std/num/struct.ParseIntError.html
[result]: https://doc.rust-lang.org/std/result/enum.Result.html

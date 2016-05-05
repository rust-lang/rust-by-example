Panicking on `unwrap()` in the previous example gave us an unhelpful error message.
To avoid that, we need to be more specific about the return type. In that example, 
recall that the regular element is of type `i32`. To determine the `Err` type, we 
look to `parse()`. `parse()` is implemented with the [`FromStr trait`][from_str] 
for [`i32`][i32]. As a result, the `Err` type is specified as [`ParseIntError`][parse_int_error].

In the example below, note that using the straightforward `match` statement leads to 
more cumbersome code. As it turns out, the `map` method we used with `Option` 
is also implemented for `Result`.

{result.play}

Much like `Option`, `Result` implements combinators besides `map`, such as `and_then`
and `unwrap_or`. This even includes those that specifically handle errors, like `map_err`.
[`Result`][result] contains the complete listing.

### See also:

[`i32`][i32], [`FromStr`][from_str], and [`ParseIntErr`][parse_int_error]

[result]: http://doc.rust-lang.org/std/result/enum.Result.html
[parse_int_error]: http://doc.rust-lang.org/std/num/struct.ParseIntError.html
[from_str]: http://doc.rust-lang.org/std/str/trait.FromStr.html
[i32]: http://doc.rust-lang.org/std/primitive.i32.html

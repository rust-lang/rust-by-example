To avoid the `unwrap()` in the previous example, we will have to rewrite the example to be
specific about what type it returns. In this case, the regular element should definitely
be `i32` but what about the `Err` type? Well, `parse()` is implemented with the
[`FromStr trait`][from_str] for [`i32`][i32]. That implementation specifies the
`Err` type as [`ParseIntError`][parse_int_error].

{result.play}

Similar to `Option`, `Result` has many other combinators besides `map` such as `and_then`
and `unwrap_or`; even ones to handle the errors specifically such as `map_err`.
`Result` contains the complete listing.

### See also:

[`i32`][i32], [`FromStr`][from_str], [`ParseIntErr`][parse_int_error], and
[`Result`][result]

[result]: http://doc.rust-lang.org/std/result/enum.Result.html
[parse_int_error]: http://doc.rust-lang.org/std/num/struct.ParseIntError.html
[from_str]: http://doc.rust-lang.org/std/str/trait.FromStr.html
[i32]: http://doc.rust-lang.org/std/primitive.i32.html

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

{result_map.play}

[parse]: https://doc.rust-lang.org/std/primitive.str.html#method.parse
[from_str]: http://doc.rust-lang.org/std/str/trait.FromStr.html
[i32]: http://doc.rust-lang.org/std/primitive.i32.html
[parse_int_error]: http://doc.rust-lang.org/std/num/struct.ParseIntError.html
[result]: http://doc.rust-lang.org/std/result/enum.Result.html

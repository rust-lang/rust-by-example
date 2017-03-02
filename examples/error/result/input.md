[`Result`][result] is a richer version of the [`Option`][option] type that
describes possible *error* instead of possible *absence*.

That is, `Result<T, E>` could have one of two outcomes:

* `Ok<T>`: An element `T` was found
* `Err<E>`: An error was found with element `E`

By convention, the expected outcome is `Ok` while the unexpected outcome is `Err`.

Like `Option`, `Result` has many methods associated with it. `unwrap()`, for
example, either yields the element `T` or `panic`s. For case handling,
there are many combinators between `Result` and `Option` that overlap.

In working with Rust, you will likely encounter methods that return the
`Result` type, such as the [`parse()`][parse] method. It might not always
be possible to parse a string into the other type, so `parse()` returns a
`Result` indicating possible failure.

Let's see what happens when we successfully and unsuccessfully `parse()` a string:

{result.play}

In the unsuccessful case, `parse()` leaves us with an error for `unwrap()`
to `panic` on. Additionally, the `panic` exits our program and provides an
unpleasant error message.

To improve the quality of our error message, we should be more specific
about the return type and consider explicitly handling the error.

[option]: http://doc.rust-lang.org/std/option/enum.Option.html
[result]: http://doc.rust-lang.org/std/result/enum.Result.html
[parse]: https://doc.rust-lang.org/std/primitive.str.html#method.parse

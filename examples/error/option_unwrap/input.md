In the last example, we showed that we can induce program failure at will.
We told our program to `panic` if the princess received an inappropriate
gift - a snake. But what if the princess expected a gift and didn't receive
one? That case would be just as bad, so it needs to be handled!

We *could* test this against the null string (`""`) as we do with a snake.
Since we're using Rust, let's instead have the compiler point out cases
where there's no gift.

An `enum` called `Option<T>` in the `std` library is used when absence is a
possibility. It manifests itself as one of two "options":

* `Some(T)`: An element of type `T` was found
* `None`: No element was found

These cases can either be explicitly handled via `match` or implicitly with
`unwrap`. Implicit handling will either return the inner element or `panic`.

Note that it's possible to manually customize `panic` with [expect][expect],
but `unwrap` otherwise leaves us with a less meaningful output than explicit
handling. In the following example, explicit handling yields a more
controlled result while retaining the option to `panic` if desired.

{option_unwrap.play}

[expect]: http://doc.rust-lang.org/std/option/enum.Option.html#method.expect

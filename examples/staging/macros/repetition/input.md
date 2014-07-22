Macros can be defined to handle indefinite arity (any number of arguments).
This can be accomplished using the *repetition* syntax and the `+`
and `*` operators. This is what the syntax looks like:

On the pattern side:

* `($($x:expr),+)`: matches a list (with at least one element) of `expr`s
  separated by commas. Examples: `('a', 'b')` and `(1 + 2, 3 * 4, 5 - 6)`

* `($($y:ident),*)`: matches a list of `ident`s separated by commas, but also
  handles the zero arity case. Examples: `()` and `(foo, bar)`

* Neither of these two patterns will match a list that has a trailing comma. To
  allow trailing commas, you can use this pattern: `($($z:expr),+,)`

On the expansion side:

* `[$($x),+]`: will expand the input arguments (the `$x`s) into an array.
  Following the previous example: `['a', 'b']` and `[1 + 2, 3 * 4, 5 - 6]`

For our example, we'll make a `min!` macro that can take any number of
arguments and will return the smallest one. Under the hood it will use the
`min` function that compares *two* arguments.

{repeat.play}

If you check the expansion you'll see this expression in the place of the last
macro:

``` rust
std::cmp::min(5u, std::cmp::min(2u * 3, 4u))
```

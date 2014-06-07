This is a summary of the primitive types in Rust:

* signed integers: `i8`, `i16`, `i32`, `i64` and `int` (machine word size)
* unsigned integers: `u8`, `u16`, `u32`, `u64` and `uint` (machine word size)
* floating point: `f32`, `f64`
* `char` Unicode scalar values like `'a'`, `'α'` and `'∞'`
* `bool` either `true` or `false`
* and the unit type `()`, whose only value is also `()`

Numeric literals can suffix their type to indicate their precision, with the
exception of `uint` that uses the `u` suffix and `int` that uses the `i`
suffix.

Integers can alternatively be expressed using hexadecimal, octal or binary
notation using either of these prefixes: `0x`, `0o` or `0b`.

You can also insert underscores in your literals for improved readability, e.g.
`1_000_000` is the same as `1000000`.

The operators available and operator precedence are similar to other
[C-like languages](https://en.wikipedia.org/wiki/Operator_precedence#Programming_languages)

{literals.play}

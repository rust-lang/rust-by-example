# Primitives

Rust provides access to a wide variety of `primitives`. A sample includes:

* signed integers: `i8`, `i16`, `i32`, `i64` and `isize` (pointer size)
* unsigned integers: `u8`, `u16`, `u32`, `u64` and `usize` (pointer size)
* floating point: `f32`, `f64`
* `char` Unicode scalar values like `'a'`, `'α'` and `'∞'` (4 bytes each)
* `bool` either `true` or `false`
* and the unit type `()`, whose only value is also `()`
* arrays like `[1, 2, 3]`
* tuples like `(1, true)`

Variables can always be *type annotated*. Numbers may additionally be
annotated via a *suffix* or *by default*. Integers default to `i32` and
floats to `f64`.

```rust,editable
fn main() {
    // Variables can be type annotated.
    let logical: bool = true;

    let a_float: f64 = 1.0;  // Regular annotation
    let an_integer   = 5i32; // Suffix annotation

    // Or a default will be used.
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`

    let mut mutable = 12; // Mutable `i32`.

    // Error! The type of a variable can't be changed
    mutable = true;
}
```

### See also:

[the `std` library][std]

[std]: https://doc.rust-lang.org/std/

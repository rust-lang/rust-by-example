Rust provides type safety via its static type-checker. Variables can be type
annotated when declared. However, in most cases, the compiler will be able to
infer the type of the variable from the context, heavily reducing the
annotation burden.

{type.play}

This is a summary of the primitive types in Rust:

* signed integers: `i8`, `i16`, `i32`, `i64` and `isize` (pointer size)
* unsigned integers: `u8`, `u16`, `u32`, `u64` and `usize` (pointer size)
* floating point: `f32`, `f64`
* `char` Unicode scalar values like `'a'`, `'α'` and `'∞'` (4 bytes each)
* `bool` either `true` or `false`
* and the unit type `()`, whose only value is also `()`

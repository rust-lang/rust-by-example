Conditional compilation is possible through two different operators:

* the `cfg` attribute: `#[cfg(...)]` in attribute position
* the `cfg!` macro: `cfg!(...)` in boolean expressions

Both utilize identical argument syntax.

{cfg.play}

### See also:

[the reference][ref], [`cfg!`][cfg], and [macros][macros].

[cfg]: http://doc.rust-lang.org/std/macro.cfg!.html
[macros]: /macros.html
[ref]: http://doc.rust-lang.org/reference.html#conditional-compilation

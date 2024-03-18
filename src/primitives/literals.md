# Literals and operators

Integers `1`, floats `1.2`, characters `'a'`, strings `"abc"`, booleans `true`
and the unit type `()` can be expressed using literals.

Integers can, alternatively, be expressed using hexadecimal, octal or binary
notation using these prefixes respectively: `0x`, `0o` or `0b`.

Underscores can be inserted in numeric literals to improve readability, e.g.
`1_000` is the same as `1000`, and `0.000_001` is the same as `0.000001`.

Rust also supports scientific [E-notation][enote], e.g. `1e6`, `7.6e-4`. The
associated type is `f64`.

We need to tell the compiler the type of the literals we use. For now, we'll use
the `u32` suffix to indicate that the literal is an unsigned 32-bit integer, and
the `i32` suffix to indicate that it's a signed 32-bit integer.

The operators available and their precedence [in Rust][rust op-prec] are similar
to other [C-like languages][op-prec].

```rust,editable
fn main() {
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important

    // Scientific notation
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
}
```

[enote]: https://en.wikipedia.org/wiki/Scientific_notation#E_notation
[rust op-prec]: https://doc.rust-lang.org/reference/expressions.html#expression-precedence
[op-prec]: https://en.wikipedia.org/wiki/Operator_precedence#Programming_languages

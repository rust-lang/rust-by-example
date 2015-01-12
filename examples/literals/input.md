Integers `1`, floats `1.2`, characters `'a'`, strings `"abc"`, booleans `true`
and the unit type `()` can be expressed using literals.

Integers can, alternatively, be expressed using hexadecimal, octal or binary
notation using either of these prefixes: `0x`, `0o` or `0b`.

Underscores can be inserted in numeric literals to improve readability, e.g.
`1_000` is the same as `1000`, and `0.000_001` is the same as `0.000001`.

We need to tell the compiler the type of the literals we use. For now,
we'll use the `u` suffix to indicate that the literal is an unsigned integer,
and the `i` suffix to indicate that it's a signed integer. We'll cover the type
system in [another chapter][type], and give more details about type
annotating literals in [their own section][type-literal].

The operators available and their precedence are similar to other
[C-like languages][op-prec].

{literals.play}

[op-prec]: https://en.wikipedia.org/wiki/Operator_precedence#Programming_languages
[type]: /type.html
[type-literal]: /type/literals.html

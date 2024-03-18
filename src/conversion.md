# Conversion

Primitive types can be converted to each other through [casting].

Rust addresses conversion between custom types (i.e., `struct` and `enum`) by
the use of [traits]. The generic conversions will use the [`From`] and [`Into`]
traits. However there are more specific ones for the more common cases, in
particular when converting to and from `String`s.

[casting]: types/cast.md
[traits]: trait.md
[`From`]: https://doc.rust-lang.org/std/convert/trait.From.html
[`Into`]: https://doc.rust-lang.org/std/convert/trait.Into.html

We've seen that formatting is specified via a *format string*:

* `format!("{}", foo)` -> `"3735928559"`
* `format!("0x{:X}", foo)` ->
  [`"0xDEADBEEF"`][deadbeef]
* `format!("0o{:o}", foo)` -> `"0o33653337357"`

The same variable (`foo`) can be formatted differently depending on which
*argument type* is used: `X` vs `o` vs *unspecified*.

This formatting functionality is implemented via traits, and there is one trait
for each argument type. The most common formatting trait is `Display`, which
handles cases where the argument type is left unspecified: `{}` for instance.

{show.play}

You can view a [full list of formatting traits][fmt_traits] and their argument
types in the [`std::fmt`][fmt] documentation.

### Activity
Add an implementation of the `fmt::Display` trait for the `Color` struct above
so that the output displays as:

```
RGB (128, 255, 90) 0x80FF5A
RGB (0, 3, 254) 0x0003FE
RGB (0, 0, 0) 0x000000
```
Two hints if you get stuck:
 * You [may need to list each color more than once][argument_types],
 * You can [pad with zeros to a width of 2][fmt_width] with `:02`.

### See also
[`std::fmt`][fmt]

[argument_types]: http://doc.rust-lang.org/std/fmt/#argument-types
[deadbeef]: https://en.wikipedia.org/wiki/Deadbeef#Magic_debug_values
[fmt]: http://doc.rust-lang.org/std/fmt/
[fmt_traits]: http://doc.rust-lang.org/std/fmt/#formatting-traits
[fmt_width]: http://doc.rust-lang.org/std/fmt/#width

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

Here's the full list of formatting traits and their respective argument types:

* *unspecified* -> `Display`
* `?` -> `Debug`
* `o` -> `Octal`
* `x` -> `LowerHex`
* `X` -> `UpperHex`
* `p` -> `Pointer`
* `b` -> `Binary`
* `e` -> `LowerExp`
* `E` -> `UpperExp`

[deadbeef]: https://en.wikipedia.org/wiki/Deadbeef#Magic_debug_values

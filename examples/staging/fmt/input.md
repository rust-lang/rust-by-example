We've seen that formatting is specified via a *format string*:

* `format!("{}", foo)` -> `"3735928559"`
* `format!("0x{:X}", foo)` ->
  [`"0xDEADBEEF"`][deadbeef]
* `format!("0o{:o}", foo)` -> `"0o33653337357"`

The same variable (`foo`) can be formatted differently depending on which
*argument type* is used: `X` vs `o` vs *unspecified*.

This formatting functionality is implemented via traits, and there is one trait
for each argument type. The most common formatting trait is `Show`, which
handles cases where the argument type is left unspecified: `{}` for instance.

{show.play}

Here's the full list of formatting traits and their respective argument types:

* *unspecified* -> `Show`
* `d` and `i` -> `Signed`
* `u` -> `Unsigned`
* `b` -> `Bool`
* `c` -> `Char`
* `o` -> `Octal`
* `x` -> `LowerHex`
* `X` -> `UpperHex`
* `s` -> `String`
* `p` -> `Pointer`
* `t` -> `Binary`
* `f` -> `Float`
* `e` -> `LowerExp`
* `E` -> `UpperExp`
* `?` -> `Poly`

[deadbeef]: https://en.wikipedia.org/wiki/Deadbeef#Magic_debug_values

Printing is handled by a series of [macros][macros] defined in [`std::fmt`][fmt]
some of which include:

* `format!`: write formatted text to [`String`][string]
* `print!`: same as `format!` but the text is printed to the console.
* `println!`: same as `print!` but a newline is appended.

All parse text in the same fashion. A plus is that the formatting correctness will
be checked at compile time.

{print.play}

[`std::fmt`][fmt] contains many [`trait`s][traits] which govern the display
of text. The base form of two important ones are listed below:

* `fmt::Debug`: Uses the `{:?}` marker. Format text for debugging purposes.
* `fmt::Display`: Uses the `{}` marker. Format text in a more elegant, user
friendly fashion.

Here, `fmt::Display` was used because the std library provides implementations
for these types. To print text for custom types, more steps are required.

### See also:

[`std::fmt`][fmt], [macros][macros], [`struct`][structs],
and [`trait`s][traits]

[fmt]: http://doc.rust-lang.org/std/fmt/
[macros]: /macros.html
[string]: /std/str.html
[structs]: /custom_types/structs.html
[traits]: /trait.html

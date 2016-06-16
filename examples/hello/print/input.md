Printing is handled by a series of [`macros`][macros] defined in [`std::fmt`][fmt]
some of which include:

* `format!`: write formatted text to [`String`][string]
* `print!`: same as `format!` but the text is printed to the console.
* `println!`: same as `print!` but a newline is appended.

All parse text in the same fashion. A plus is that the formatting correctness will
be checked at compile time.

{print.play}

[`std::fmt`][fmt] contains many [`traits`][traits] which govern the display
of text. The base form of two important ones are listed below:

* `fmt::Debug`: Uses the `{:?}` marker. Format text for debugging purposes.
* `fmt::Display`: Uses the `{}` marker. Format text in a more elegant, user
friendly fashion.

Here, `fmt::Display` was used because the std library provides implementations
for these types. To print text for custom types, more steps are required.

### Activities

 * Fix the two issues in the above code (see FIXME) so that it runs without
   error.
 * Add a `println!` macro that prints: `Pi is roughly 3.142` by controlling
   the number of decimal places shown. For the purposes of this exercise,
   use `let pi = 3.141592` as an estimate for Pi. (Hint: you may need to
   check the [`std::fmt`][fmt] documentation for setting the number of
   decimals to display)

### See also

[`std::fmt`][fmt], [`macros`][macros], [`struct`][structs],
and [`traits`][traits]

[fmt]: http://doc.rust-lang.org/std/fmt/
[macros]: /macros.html
[string]: /std/str.html
[structs]: /custom_types/structs.html
[traits]: /trait.html

# Formatted print

Printing is handled by a series of [`macros`][macros] defined in [`std::fmt`][fmt]
some of which include:

* `format!`: write formatted text to [`String`][string]
* `print!`: same as `format!` but the text is printed to the console (io::stdout).
* `println!`: same as `print!` but a newline is appended.
* `eprint!`: same as `format!` but the text is printed to the standard error (io::stderr).
* `eprintln!`: same as `eprint!`but a newline is appended.

All parse text in the same fashion. As a plus, Rust checks formatting
correctness at compile time.

```rust,editable,ignore,mdbook-runnable
fn main() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // Without a suffix, 31 becomes an i32. You can change what type 31 is
    // by providing a suffix. The number 31i64 for example has the type i64.

    // There are various optional patterns this works with. Positional
    // arguments can be used.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // Special formatting can be specified after a `:`.
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}", number=1, width=6);

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:>0width$}", number=1, width=6);

    // Rust even checks to make sure the correct number of arguments are
    // used.
    println!("My name is {0}, {1} {0}", "Bond");
    // FIXME ^ Add the missing argument: "James"

    // Create a structure named `Structure` which contains an `i32`.
    #[allow(dead_code)]
    struct Structure(i32);

    // However, custom types such as this structure require more complicated
    // handling. This will not work.
    println!("This struct `{}` won't print...", Structure(3));
    // FIXME ^ Comment out this line.
}
```

[`std::fmt`][fmt] contains many [`traits`][traits] which govern the display
of text. The base form of two important ones are listed below:

* `fmt::Debug`: Uses the `{:?}` marker. Format text for debugging purposes.
* `fmt::Display`: Uses the `{}` marker. Format text in a more elegant, user
friendly fashion.

Here, we used `fmt::Display `because the std library provides implementations
for these types. To print text for custom types, more steps are required.

Implementing the `fmt::Display` trait automatically implements the
[`ToString`] trait which allows us to [convert] the type to [`String`][string].

### Activities

 * Fix the two issues in the above code (see FIXME) so that it runs without
   error.
 * Add a `println!` macro that prints: `Pi is roughly 3.142` by controlling
   the number of decimal places shown. For the purposes of this exercise,
   use `let pi = 3.141592` as an estimate for pi. (Hint: you may need to
   check the [`std::fmt`][fmt] documentation for setting the number of
   decimals to display)

### See also:

[`std::fmt`][fmt], [`macros`][macros], [`struct`][structs],
and [`traits`][traits]

[fmt]: https://doc.rust-lang.org/std/fmt/
[macros]: ../macros.md
[string]: ../std/str.md
[structs]: ../custom_types/structs.md
[traits]: ../trait.md
[`ToString`]: https://doc.rust-lang.org/std/string/trait.ToString.html
[convert]: ../conversion/string.md

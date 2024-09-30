# `macro_rules!`

Rust provides a powerful macro system that allows metaprogramming. As you've
seen in previous chapters, macros look like functions, except that their name
ends with a bang `!`, but instead of generating a function call, macros are
expanded into source code that gets compiled with the rest of the program.
However, unlike macros in C and other languages, Rust macros are expanded into
abstract syntax trees, rather than string preprocessing, so you don't get
unexpected precedence bugs.

Macros are created using the `macro_rules!` macro.

```rust,editable
// This is a simple macro named `say_hello`.
macro_rules! say_hello {
    // `()` indicates that the macro takes no argument.
    () => {
        // The macro will expand into the contents of this block.
        println!("Hello!")
    };
}

fn main() {
    // This call will expand into `println!("Hello!")`
    say_hello!()
}
```

So why are macros useful?

1. Don't repeat yourself. There are many cases where you may need similar
   functionality in multiple places but with different types. Often, writing a
   macro is a useful way to avoid repeating code. (More on this later)

2. Domain-specific languages. Macros allow you to define special syntax for a
   specific purpose. (More on this later)

3. Variadic interfaces. Sometimes you want to define an interface that takes a
   variable number of arguments. An example is `println!` which could take any
   number of arguments, depending on the format string. (More on this later)

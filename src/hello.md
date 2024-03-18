# Hello World

This is the source code of the traditional Hello World program.

```rust,editable
// This is a comment, and is ignored by the compiler.
// You can test this code by clicking the "Run" button over there ->
// or if you prefer to use your keyboard, you can use the "Ctrl + Enter"
// shortcut.

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

// This is the main function.
fn main() {
    // Statements here are executed when the compiled binary is called.

    // Print text to the console.
    println!("Hello World!");
}
```

`println!` is a [*macro*][macros] that prints text to the console.

A binary can be generated using the Rust compiler: `rustc`.

```bash
$ rustc hello.rs
```

`rustc` will produce a `hello` binary that can be executed.

```bash
$ ./hello
Hello World!
```

### Activity

Click 'Run' above to see the expected output. Next, add a new line with a second
`println!` macro so that the output shows:

```text
Hello World!
I'm a Rustacean!
```

[macros]: macros.md

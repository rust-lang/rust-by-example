# Comments

Any program requires comments, and Rust supports a few different varieties:

- *Regular comments* which are ignored by the compiler:
  - `// Line comments which go to the end of the line.`
  - `/* Block comments which go to the closing delimiter. */`
- *Doc comments* which are parsed into HTML library [documentation][docs]:
  - `/// Generate library docs for the following item.`
  - `//! Generate library docs for the enclosing item.`

```rust,editable
fn main() {
    // This is an example of a line comment.
    // There are two slashes at the beginning of the line.
    // And nothing written after these will be read by the compiler.

    // println!("Hello, world!");

    // Run it. See? Now try deleting the two slashes, and run it again.

    /*
     * This is another type of comment, a block comment. In general,
     * line comments are the recommended comment style. But block comments
     * are extremely useful for temporarily disabling chunks of code.
     * /* Block comments can be /* nested, */ */ so it takes only a few
     * keystrokes to comment out everything in this main() function.
     * /*/*/* Try it yourself! */*/*/
     */

    /*
    Note: The previous column of `*` was entirely for style. There's
    no actual need for it.
    */

    // You can manipulate expressions more easily with block comments
    // than with line comments. Try deleting the comment delimiters
    // to change the result:
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);
}
```

### See also:

[Library documentation][docs]

[docs]: ../meta/doc.md

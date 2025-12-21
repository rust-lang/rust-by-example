# Comments

Any program requires comments, and Rust supports
a few different varieties:

## Regular Comments

These are ignored by the compiler:

* **Line comments**: Start with `//` and continue to the end of the line
* **Block comments**: Enclosed in `/* ... */` and can span multiple lines

## Documentation Comments (Doc Comments) which are parsed into HTML library [documentation][docs]:

 - `///` - Generates docs for the item that follows it
- `//!` - Generates docs for the enclosing item (typically used at the top of a file or module)
```rust,editable

fn main() {
    // Line comments start with two slashes.
    // Everything after the slashes is ignored by the compiler.

    // Example: This line won't execute
    // println!("Hello, world!");

    // Try removing the slashes above and running the code again.

    /*
     * Block comments are useful for temporarily disabling code.
     * They can also be nested: /* like this */ which makes it easy
     * to comment out large sections quickly.
     */

    /*
    Note: The asterisk column on the left is just for style - 
    it's not required by the language.
    */

    // Block comments make it easy to toggle code on/off by adding
    // or removing just one slash:

    /* <- Add a '/' here to uncomment the entire block below

    println!("Now");
    println!("everything");
    println!("executes!");
    // Line comments inside remain unaffected

    // */

    // Block comments can also be used within expressions:
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);
}
```

### See also:

[Library documentation][docs]

[docs]: ../meta/doc.md

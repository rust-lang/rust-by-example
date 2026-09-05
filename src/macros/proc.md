# Procedural macros

`macro_rules!` patterns match syntax, but some code generation needs
the full power of Rust code: reading struct fields, deriving trait
implementations, or generating new items from attributes. Procedural
macros do that — they are functions from token stream to token stream
that run at compile time. There are three kinds:

* **Derive macros** add `#[derive(...)]` implementations, e.g.
  `#[derive(Debug)]` or `serde`'s `Serialize`.
* **Attribute macros** define new attributes like `#[tokio::main]`
  that transform the annotated item.
* **Function-like macros** look like function calls, e.g.
  `sql!(SELECT * FROM posts)`, taking tokens and expanding to code.

The canonical derive example is serialization: one attribute generates
correct, field-by-field conversion code you would otherwise hand-write
and drift out of sync:

```rust,editable,ignore,mdbook-runnable
// Cargo.toml: serde = { version = "1", features = ["derive"] }
// Cargo.toml: serde_json = "1"

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() -> serde_json::Result<()> {
    let point = Point { x: 1, y: 2 };
    let json = serde_json::to_string(&point)?;
    println!("serialized: {}", json);

    let back: Point = serde_json::from_str(&json)?;
    println!("round-trip: {:?}", back);
    Ok(())
}
```

When to use which: reach for `macro_rules!` for syntax sugar inside
your crate (compact repetition, small DSLs); reach for a proc-macro
when you generate trait impls from type structure or transform
annotated items. Writing a new proc-macro means a separate crate with
`proc-macro = true` — start from an established framework crate rather
than raw token manipulation.

### See also:

[`macro_rules!`][macros], [`serde`][serde], and [the Procedural Macros
chapter of the Reference][ref].

[macros]: ../macros.md
[serde]: https://serde.rs/derive.html
[ref]: https://doc.rust-lang.org/reference/procedural-macros.html

### Exercise: Derive instead of hand-writing

Task: Replace a hand-written `Display` impl on a three-field struct with a derived serialization.

<details><summary>Hint</summary>

One attribute on the struct generates the conversion, so the manual formatting code can go away entirely.

</details>

<details><summary>Solution</summary>

```rust,editable,ignore,mdbook-runnable
// Cargo.toml: serde = { version = "1", features = ["derive"] }
// Cargo.toml: serde_json = "1"

use serde::Serialize;

// No manual `Display` needed: the derived impl tracks the fields.
#[derive(Serialize, Debug)]
struct Config {
    host: String,
    port: u16,
    tls: bool,
}

fn main() -> serde_json::Result<()> {
    let config = Config {
        host: "localhost".to_string(),
        port: 8080,
        tls: false,
    };
    println!("{}", serde_json::to_string_pretty(&config)?);
    Ok(())
}
```
</details>

# constants

Rust has two different types of constants which can be declared in any scope
including global. Both require explicit type annotation:

- `const`: An unchangeable value (the common case).
- `static`: A possibly mutable variable with [`'static`][static] lifetime. The
  static lifetime is inferred and does not have to be specified. Accessing or
  modifying a mutable static variable is [`unsafe`][unsafe].

```rust,editable,ignore,mdbook-runnable
// Globals are declared outside all other scopes.
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Error! Cannot modify a `const`.
    THRESHOLD = 5;
    // FIXME ^ Comment out this line
}
```

### See also:

[The `const`/`static` RFC](https://github.com/rust-lang/rfcs/blob/master/text/0246-const-vs-static.md),
[`'static` lifetime][static]

[static]: ../scope/lifetime/static_lifetime.md
[unsafe]: ../unsafe.md

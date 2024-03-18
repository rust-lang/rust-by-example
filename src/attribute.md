# Attributes

An attribute is metadata applied to some module, crate or item. This metadata
can be used to/for:

<!-- TODO: Link these to their respective examples -->

- [conditional compilation of code][cfg]
- [set crate name, version and type (binary or library)][crate]
- disable [lints][lint] (warnings)
- enable compiler features (macros, glob imports, etc.)
- link to a foreign library
- mark functions as unit tests
- mark functions that will be part of a benchmark
- [attribute like macros][macros]

Attributes look like `#[outer_attribute]` or `#![inner_attribute]`, with the
difference between them being where they apply.

- `#[outer_attribute]` applies to the [item][item] immediately following it.
  Some examples of items are: a function, a module declaration, a constant, a
  structure, an enum. Here is an example where attribute `#[derive(Debug)]`
  applies to the struct `Rectangle`:
  ```rust
  #[derive(Debug)]
  struct Rectangle {
      width: u32,
      height: u32,
  }
  ```

- `#![inner_attribute]` applies to the enclosing [item][item] (typically a
  module or a crate). In other words, this attribute is interpreted as applying
  to the entire scope in which it's placed. Here is an example where
  `#![allow(unused_variables)]` applies to the whole crate (if placed in
  `main.rs`):
  ```rust
  #![allow(unused_variables)]

  fn main() {
      let x = 3; // This would normally warn about an unused variable.
  }
  ```

Attributes can take arguments with different syntaxes:

- `#[attribute = "value"]`
- `#[attribute(key = "value")]`
- `#[attribute(value)]`

Attributes can have multiple values and can be separated over multiple lines,
too:

```rust,ignore
#[attribute(value, value2)]


#[attribute(value, value2, value3,
            value4, value5)]
```

[cfg]: attribute/cfg.md
[crate]: attribute/crate.md
[item]: https://doc.rust-lang.org/stable/reference/items.html
[lint]: https://en.wikipedia.org/wiki/Lint_%28software%29
[macros]: https://doc.rust-lang.org/book/ch19-06-macros.html#attribute-like-macros

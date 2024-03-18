# Documentation

Use `cargo doc` to build documentation in `target/doc`, `cargo doc --open` will
automatically open it in your web browser.

Use `cargo test` to run all tests (including documentation tests), and
`cargo test --doc` to only run documentation tests.

These commands will appropriately invoke `rustdoc` (and `rustc`) as required.

## Doc comments

Doc comments are very useful for big projects that require documentation. When
running `rustdoc`, these are the comments that get compiled into documentation.
They are denoted by a `///`, and support [Markdown].

````rust,editable,ignore
#![crate_name = "doc"]

/// A human being is represented here
pub struct Person {
    /// A person must have a name, no matter how much Juliet may hate it
    name: String,
}

impl Person {
    /// Creates a person with the given name.
    ///
    /// # Examples
    ///
    /// ```
    /// // You can have rust code between fences inside the comments
    /// // If you pass --test to `rustdoc`, it will even test it for you!
    /// use doc::Person;
    /// let person = Person::new("name");
    /// ```
    pub fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
        }
    }

    /// Gives a friendly hello!
    ///
    /// Says "Hello, [name](Person::name)" to the `Person` it is called on.
    pub fn hello(&self) {
        println!("Hello, {}!", self.name);
    }
}

fn main() {
    let john = Person::new("John");

    john.hello();
}
````

To run the tests, first build the code as a library, then tell `rustdoc` where
to find the library so it can link it into each doctest program:

```shell
$ rustc doc.rs --crate-type lib
$ rustdoc --test --extern doc="libdoc.rlib" doc.rs
```

## Doc attributes

Below are a few examples of the most common `#[doc]` attributes used with
`rustdoc`.

### `inline`

Used to inline docs, instead of linking out to separate page.

```rust,ignore
#[doc(inline)]
pub use bar::Bar;

/// bar docs
pub mod bar {
    /// the docs for Bar
    pub struct Bar;
}
```

### `no_inline`

Used to prevent linking out to separate page or anywhere.

```rust,ignore
// Example from libcore/prelude
#[doc(no_inline)]
pub use crate::mem::drop;
```

### `hidden`

Using this tells `rustdoc` not to include this in documentation:

```rust,editable,ignore
// Example from the futures-rs library
#[doc(hidden)]
pub use self::async_await::*;
```

For documentation, `rustdoc` is widely used by the community. It's what is used
to generate the [std library docs](https://doc.rust-lang.org/std/).

### See also:

- [The Rust Book: Making Useful Documentation Comments][book]
- [The rustdoc Book][rustdoc-book]
- [The Reference: Doc comments][ref-comments]
- [RFC 1574: API Documentation Conventions][api-conv]
- [RFC 1946: Relative links to other items from doc comments (intra-rustdoc links)][intra-links]
- [Is there any documentation style guide for comments? (reddit)][reddit]

[markdown]: https://en.wikipedia.org/wiki/Markdown
[book]: https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#making-useful-documentation-comments
[ref-comments]: https://doc.rust-lang.org/stable/reference/comments.html#doc-comments
[rustdoc-book]: https://doc.rust-lang.org/rustdoc/index.html
[api-conv]: https://rust-lang.github.io/rfcs/1574-more-api-documentation-conventions.html#appendix-a-full-conventions-text
[intra-links]: https://rust-lang.github.io/rfcs/1946-intra-rustdoc-links.html
[reddit]: https://www.reddit.com/r/rust/comments/ahb50s/is_there_any_documentation_style_guide_for/

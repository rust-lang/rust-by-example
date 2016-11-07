Doc comments are very useful for big projects that require documentation. When
running [Rustdoc][1], these are the comments that get compiled into
documentation. They are denoted by a `///`, and support [Markdown][2].

{doc.play}

To run the tests, first build the code as a library, then tell rustdoc where
to find the library so it can link it into each doctest program:

```
rustc doc.rs --crate-type lib
rustdoc --test --extern doc="libdoc.rs"
```

(When you run `cargo test` on a library crate, Cargo will automatically
generate and run the correct rustc and rustdoc commands.)

[1]: http://doc.rust-lang.org/book/documentation.html
[2]: https://en.wikipedia.org/wiki/Markdown

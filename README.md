# Rust by Example

## What's this?

This is the source code of the
[Rust by example](http://rustbyexample.github.io/) website!

## How to contribute

### An example has a typo, it's outdated, not clear or the information is wrong

Please open an issue explaining the problem, or see below how to modify an
example.

### I like to see an example about ${topic}

Leave a comment in
[issue #1](https://github.com/japaric/rust-by-example/issues/1).

### I want to add/modify an example

You'll need these tools:

* [Rust](http://www.rust-lang.org/) (\o/)
* [gitbook](http://www.gitbook.io)

Each example has:

* an id, e.g. `hello`
* a title, e.g. `Hello World`
* a directory under `src`, e.g. `src/hello`
* an entry in [`src/order.json`](src/order.json), e.g.
  `{ "id": "hello", "title": "Hello World" }`
* some source code, e.g. `src/hello/hello.rs`
* an input markdown file, e.g. `src/hello/input.md`

Take a look at the source code of the [Hello World example](src/hello) as a
reference.

Note that the markdown doesn't need to copy the rust code, instead it just has
to reference the filename of the rust code, as in `{hello.rs}`.

`make` will merge `input.md` and the rust code into a `README.md` file
located under `output/examples/${id}`, AND generate the `SUMMARY.md` of the
book, which lists the examples in the same order they appear in `order.json`.

`make serve` will publish the book under `localhost:4000`, there you can check
that the example looks correct.

`make test` will compile all the examples and point out compiler errors if any.

After checking that the changes look correct, you can send a pull request.

**Note**: If you're working in a new example, it would be extreme helpful to
open an issue mentioning on which topic you are working on, to let other people
know which topics are being worked on.

## License

Rust by example is dual licensed under the Apache 2.0 license and the MIT
license.

See LICENSE-APACHE and LICENSE-MIT for more details.

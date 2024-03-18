# Rust By Example

[![Build Status][travis-badge]][travis-repo]

[travis-badge]: https://travis-ci.com/rust-lang/rust-by-example.svg?branch=master
[travis-repo]: https://travis-ci.com/rust-lang/rust-by-example

Learn Rust with examples (Live code editor included)

## Using

If you'd like to read Rust by Example, you can visit
<https://doc.rust-lang.org/rust-by-example/> to read it online.

If you'd like to read it locally, [install Rust], and then:

```bash
git clone https://github.com/rust-lang/rust-by-example
cd rust-by-example
cargo install mdbook
mdbook build
mdbook serve
```

[install Rust]: https://www.rust-lang.org/tools/install

To be able to run the examples, you must be connected to the internet; you can
read all content offline, however!

**The following warnings can be ignored safely.**

```
[WARN] (mdbook::preprocess::cmd): The command wasn't found, is the "gettext" preprocessor installed?
[WARN] (mdbook::preprocess::cmd):   Command: mdbook-gettext
```

### Using translated version

If there is a translated resource in `po/` directory, it can be specified
through `MDBOOK_BOOK__LANGUAGE` like below:

```bash
git clone https://github.com/rust-lang/rust-by-example
cd rust-by-example
cargo install mdbook
MDBOOK_BOOK__LANGUAGE=ja mdbook build
MDBOOK_BOOK__LANGUAGE=ja mdbook serve
```

## Contributing

Please see the [CONTRIBUTING.md] file for more details.

[CONTRIBUTING.md]: https://github.com/rust-lang/rust-by-example/blob/master/CONTRIBUTING.md

## Translating

Please see the [TRANSLATING.md] file for more details.

[TRANSLATING.md]: https://github.com/rust-lang/rust-by-example/blob/master/TRANSLATING.md

### Translating guide for each languages

- Japanese/日本語: [TRANSLATING_JA.md]

[TRANSLATING_JA.md]: https://github.com/rust-lang/rust-by-example/blob/master/TRANSLATING_JA.md

## Translations to other languages

- [Bulgarian](https://github.com/kberov/rust-by-example-bg)
- [Chinese](https://github.com/rust-lang-cn/rust-by-example-cn)
- [Japanese](https://github.com/rust-lang-ja/rust-by-example-ja)
- [French](https://github.com/Songbird0/FR_RBE)
- [Russian](https://github.com/ruRust/rust-by-example)
- [Vietnamese](https://github.com/EyesCrypto-Insights/rust-by-example-vn)
- [Portuguese](https://github.com/nazarepiedady/rust-com-exemplos)

## License

Rust by Example is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  <http://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Rust by Example by you, as defined in the Apache-2.0 license,
shall be dually licensed as above, without any additional terms or conditions.

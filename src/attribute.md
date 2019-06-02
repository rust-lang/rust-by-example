# Attributes

An attribute is metadata applied to some module, crate or item. This metadata
can be used to/for:

<!-- TODO: Link these to their respective examples -->

* [conditional compilation of code][cfg]
* [set crate name, version and type (binary or library)][crate]
* disable [lints][lint] (warnings)
* enable compiler features (macros, glob imports, etc.)
* link to a foreign library
* mark functions as unit tests
* mark functions that will be part of a benchmark

When attributes apply to a whole crate, their syntax is `#![crate_attribute]`,
and when they apply to a module or item, the syntax is `#[item_attribute]`
(notice the missing bang `!`).

Attributes can take arguments with different syntaxes:

* `#[attribute = "value"]`
* `#[attribute(key = "value")]`
* `#[attribute(value)]`

Attributes can have multiple values and can be separated over multiple lines, too:

```rust,ignore
#[attribute(value, value2)]


#[attribute(value, value2, value3,
            value4, value5)]
```

[cfg]: attribute/cfg.md
[crate]: attribute/crate.md
[lint]: https://en.wikipedia.org/wiki/Lint_%28software%29

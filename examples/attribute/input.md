An attribute is metadata applied to some module, crate or item. This metadata
can be used to/for:

* conditional compilation of code
* disable lints (warnings)
* enable compiler features (macros, glob imports, etc.)
* link to a foreign library
* mark functions as unit tests
* mark functions that will be part of a benchmark
* select crate type
* set library name and version

When attributes apply to a whole crate, their syntax is `#![crate_attribute]`,
and when they apply to a module or item, the syntax is `#[item_attribute]`
(notice the missing bang `!`). Some attributes can take key-value arguments
like `#[attribute(key = "value")]`, or string arguments like
`#[attribute = "value"]`.

Let's add metadata to the library we created in the previous chapter.

{lib.rs}

```
# we don't need the --crate-type flag this time
$ rustc lib.rs
$ ls lib*
liberty-a1e7dc98-0.1.rlib
```

Here's an example of conditional compilation.

{conditional.rs}

{conditional.play}

{conditional.out}

Some conditionals like `target_os` are implicitly provided by `rustc`, but
custom conditionals must be passed to `rustc` using the `--cfg` flag.

{custom.rs}

{custom.out}

```
$ rustc --cfg some_condition custom.rs && ./custom
condition met!
```

More usages of attributes will be covered in other chapters.

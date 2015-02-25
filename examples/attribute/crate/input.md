The `crate_type` attribute can be used to tell the compiler whether a crate is
a binary or a library (and even which type of library), and the `crate_name`
attribute can be used to set the name of the crate.

{lib.rs}

When the `crate_type` attribute is used, we no longer need to pass the
`--crate-type` flag to `rustc`.

```
$ rustc lib.rs
$ ls lib*
library.rlib
```

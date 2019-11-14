# Crates

A crate is a compilation unit in Rust. Whenever `rustc some_file.rs` is called,
`some_file.rs` is treated as the *crate file*. If `some_file.rs` has `mod`
declarations in it, then the contents of the module files would be inserted in
places where `mod` declarations in the crate file are found, *before* running
the compiler over it. In other words, modules do *not* get compiled
individually, only crates get compiled.

A crate can be compiled into a binary or into a library. By default, `rustc`
will produce a binary from a crate. This behavior can be overridden by passing
the `--crate-type` flag to `lib`.

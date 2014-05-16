A crate is a compilation unit in Rust. Whenever `rustc some_file.rs` is called,
`some_file.rs` is treated as the crate file. If `some_file.rs` has `mod`
declarations in it, then the contents of the module files will get merged with
the crate file before running the compiler over it. In other words, modules
do *not* get compiled individually, only crates get compiled.

A crate can be compiled into a binary or into a library. By default, `rustc`
will produce a binary from a crate. This behavior can be overridden passing the
`--crate-type` flag to `rustc`.

Let's create a library, and then see how to link it to another crate.

{erty.rs}

```
$ rustc --crate-type=lib erty.rs
$ ls lib*
liberty-e6eaab2e-0.0.rlib
```

Libraries get prefixed with "lib", and contain a hash and their version in
their name. The version and name of the library can be changed using
*attributes* (which are covered in the next section).

To link a crate to this new library, the `extern crate` declaration must be
used. This will not only link the library, but also import all its items
under a module named the same as the library. The visibility rules that apply
to modules also apply to libraries.

{executable.rs}

```
# -L . adds the current directory to the library search path
$ rustc -L . executable.rs && ./executable
called erty's public_function()
called erty's indirect_access(), that
> called erty's private_function()
```

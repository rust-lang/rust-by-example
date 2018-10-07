# Build Scripts

Sometimes a normal build from cargo is not enough. Perhaps your crate needs some
pre-requisites before cargo will successfully compile, things like code
generation, or some native code that needs to be compiled. To solve this problem
we have build scripts that Cargo can run.

To add a build script to your package it can either be specified in the
`Cargo.toml` as follows:

```toml
[package]
...
build = "build.rs"
```

Otherwise Cargo will look for a `build.rs` file in the project directory by
default.

## How to use a build script

The build script is simply another Rust file that will be compiled and invoked
prior to compiling anything else in the package. Hence it can be used to fulfil
pre-requisites of your crate.

Cargo provides the script with inputs via environment variables [specified
here] that can be used.

The script provides output via stdout. All lines printed are written to
`target/debug/build/<pkg>/output`. Further, lines prefixed with `cargo:` will be
interpreted by Cargo directly and hence can be used to define parameters for the
packages compilation.

For further specification and examples have a read of the [cargo specification].

[specified here]: https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts

[cargo specification]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
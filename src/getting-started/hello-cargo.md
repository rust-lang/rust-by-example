# Hello, Cargo!

Cargo is Rust’s build system and package manager. Most Rustaceans will use this
tool to manage their Rust projects because Cargo takes care of a lot of tasks
for you, such as building your code, downloading the libraries your code
depends on, and building those libraries. (We call libraries your code needs
*dependencies*.)

## Creating a Project with Cargo

To create a new project, use `cargo new`:

```text
$ cargo new hello_cargo --bin
$ cd hello_cargo
```
Cargo has created a few files for us. Open up *Cargo.toml* in your text
editor of choice. It should look similar to this:

<span class="filename">Filename: Cargo.toml</span>

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]

[dependencies]
```

This file is in the [*TOML*][toml] format, which is what Cargo uses as its
configuration format.

[toml]: https://github.com/toml-lang/toml

The first line, `[package]`, is a section heading that indicates that the
following statements are configuring a package. As we add more information to
this file, we’ll add other sections.

The last line, `[dependencies]`, is the start of a section for you to list any
of your project’s dependencies. In Rust, packages of code are referred to as
*crates*.

Now open up *src/main.rs* and take a look:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    println!("Hello, world!");
}
```

Cargo has generated a “Hello World!” for you, just like the one we wrote in
the last section.

Cargo expects your source files to live inside the *src* directory; the
top-level project directory is just for READMEs, license information,
configuration files, and anything else not related to your code.

### Building and Running a Cargo Project

You can build your project with `cargo build`:

```text
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```

But we can also use `cargo run` to compile and then run all in one go:

```text
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

Finally, there’s `cargo check`. This command will quickly check your code to
make sure that it compiles, but not bother producing an executable:

```text
$ cargo check
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

Why would you not want an executable? `cargo check` is often much faster than
`cargo build`, because it skips the entire step of producing the executable. If
you’re checking your work throughout the process of writing the code, using
`cargo check` will speed things up!

### Building for Release

When your project is finally ready for release, do this to compile your
project with optimizations:

```bash
$ cargo build --release
```

These optimizations make your Rust code run faster, but turning them on
increases the compilation time of your program.
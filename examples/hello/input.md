This is the source code of the traditional Hello World program.

{hello.play}

`println!` is a *macro* (we'll cover them later) that prints text to the
console.

A binary can be generated using the Rust compiler: `rustc`.

```
$ rustc hello.rs
```

`rustc` will produce a `hello` binary that can be executed.

```
$ ./hello
Hello World!
```

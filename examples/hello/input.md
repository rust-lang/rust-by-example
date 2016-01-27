This is the source code of the traditional Hello World program.

{hello.play}

`println!` is a [*macro*][macros] that prints text to the
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

### Activity

Click 'Run' above to see the expected output. Next, add a new
line with a second `println!` macro so that the output
shows:
```
Hello World!
I'm a Rustacean!
```

[macros]: ./macros.html

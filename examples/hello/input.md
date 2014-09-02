This is the source code of the traditional Hello World program.

You can test this code by clicking the "Run" button on the top right corner.

Or if prefer to use your keyboard, you can use the "Ctrl + Enter" shortcut.

The code is also editable.  Feel free to hack it!

You can always return to the original code by clicking the "Reset" button.

{hello.play}

`println!` is a *macro* (we'll cover them later) that prints text to the
console.

A binary can be generated using the rust compiler: `rustc`.

```
$ rustc hello.rs
```

`rustc` will produce a `hello` binary that can be executed.

```
$ ./hello
Hello World!
```

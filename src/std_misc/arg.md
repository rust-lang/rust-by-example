# Program arguments

## Standard Library

The command line arguments can be accessed using `std::env::args`, which returns
an iterator that yields a `String` for each argument:

```rust,editable
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // The first argument is the path that was used to call the program.
    println!("My path is {}.", args[0]);

    // The rest of the arguments are the passed command line parameters.
    // Call the program like this:
    //   $ ./args arg1 arg2
    println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);
}
```

```shell
$ ./args 1 2 3
My path is ./args.
I got 3 arguments: ["1", "2", "3"].
```

## Crates

Alternatively, there are numerous crates that can provide extra functionality
when creating command-line applications. One of the more popular command line
argument crates being [`clap`].

[`clap`]: https://rust-cli.github.io/book/tutorial/cli-args.html#parsing-cli-arguments-with-clap

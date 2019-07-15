# Library

Let's create a library, and then see how to link it to another crate.

```rust,ignore
pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}
```

```shell
$ rustc --crate-type=lib rary.rs
$ ls lib*
library.rlib
```

Libraries get prefixed with "lib", and by default they get named after their
crate file, but this default name can be overridden using the [`crate_name`
attribute][crate-name].

[crate-name]: ../attribute/crate.md
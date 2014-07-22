You can check the expansion of any macro by running the
`rustc --pretty expanded` command over your source file.

Let's see what the `vec!` macro expands to:

{vec.rs}

``` rust
$ rustc --pretty expanded vec.rs
#![feature(phase)]
#![no_std]
#![feature(globs)]
#[phase(plugin, link)]
extern crate std;
extern crate native;
use std::prelude::*;
fn main() {
    let v1 =
        { let mut _temp = ::std::vec::Vec::new(); _temp.push(1u); _temp };
    let v2 =
        {
            let mut _temp = ::std::vec::Vec::new();
            _temp.push(1u);
            _temp.push(2);
            _temp
        };
}
```

Soon you'll learn how to define macros that work like this one!

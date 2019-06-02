# `cfg`

Conditional compilation is possible through two different operators:

* the `cfg` attribute: `#[cfg(...)]` in attribute position
* the `cfg!` macro: `cfg!(...)` in boolean expressions

Both utilize identical argument syntax.

```rust,editable
// This function only gets compiled if the target OS is linux
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

// And this function only gets compiled if the target OS is *not* linux
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!");
}

fn main() {
    are_you_on_linux();
    
    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
}
```

### See also:

[the reference][ref], [`cfg!`][cfg], and [macros][macros].

[cfg]: https://doc.rust-lang.org/std/macro.cfg!.html
[macros]: ../macros.md
[ref]: https://doc.rust-lang.org/reference/attributes.html#conditional-compilation

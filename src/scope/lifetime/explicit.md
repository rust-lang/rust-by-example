# Explicit annotation

The borrow checker uses explicit lifetime annotations to determine how long
references should be valid. In cases where lifetimes are not elided[^1], Rust
requires explicit annotations to determine what the lifetime of a reference
should be. The syntax for explicitly annotating a lifetime uses an apostrophe
character as follows:

```rust,ignore
foo<'a>
// `foo` has a lifetime parameter `'a`
```

Similar to [closures][anonymity], using lifetimes requires generics.
Additionally, this lifetime syntax indicates that the lifetime of `foo` may not
exceed that of `'a`. Explicit annotation of a type has the form `&'a T` where
`'a` has already been introduced.

In cases with multiple lifetimes, the syntax is similar:

```rust,ignore
foo<'a, 'b>
// `foo` has lifetime parameters `'a` and `'b`
```

In this case, the lifetime of `foo` cannot exceed that of either `'a` *or* `'b`.

See the following example for explicit lifetime annotation in use:

```rust,editable,ignore,mdbook-runnable
// `print_refs` takes two references to `i32` which have different
// lifetimes `'a` and `'b`. These two lifetimes must both be at
// least as long as the function `print_refs`.
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

// A function which takes no arguments, but has a lifetime parameter `'a`.
fn failed_borrow<'a>() {
    let _x = 12;

    // ERROR: `_x` does not live long enough
    let _y: &'a i32 = &_x;
    // Attempting to use the lifetime `'a` as an explicit type annotation
    // inside the function will fail because the lifetime of `&_x` is shorter
    // than that of `_y`. A short lifetime cannot be coerced into a longer one.
}

fn main() {
    // Create variables to be borrowed below.
    let (four, nine) = (4, 9);

    // Borrows (`&`) of both variables are passed into the function.
    print_refs(&four, &nine);
    // Any input which is borrowed must outlive the borrower.
    // In other words, the lifetime of `four` and `nine` must
    // be longer than that of `print_refs`.

    failed_borrow();
    // `failed_borrow` contains no references to force `'a` to be
    // longer than the lifetime of the function, but `'a` is longer.
    // Because the lifetime is never constrained, it defaults to `'static`.
}
```

[^1]: [elision] implicitly annotates lifetimes and so is different.

### See also:

[generics][generics] and [closures][closures]

[anonymity]: ../../fn/closures/anonymity.md
[closures]: ../../fn/closures.md
[elision]: elision.md
[generics]: ../../generics.md

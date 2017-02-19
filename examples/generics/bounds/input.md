When working with generics, the type parameters often must use traits as *bounds* to
stipulate what functionality a type implements. For example, the following
example uses the trait `Display` to print and so it requires `T` to be bound
by `Display`; that is, `T` *must* implement `Display`.

```rust
// Define a function `printer` that takes a generic type `T` which
// must implement trait `Display`.
fn printer<T: Display>(t: T) {
    println!("{}", t);
}
```

Bounding restricts the generic to types that conform to the bounds. That is:

```rust
struct S<T: Display>(T);

// Error! `Vec<T>` does not implement `Display`. This
// specialization will fail.
let s = S(vec![1]);
```

Another effect of bounding is that generic instances are allowed to access the 
[methods] of traits specified in the bounds. For example:

{bounds.play}

As an additional note, [`where`][where] clauses can also be used to apply bounds in
some cases to be more expressive.

### See also:

[`std::fmt`][fmt], [`struct`s][structs], and [`trait`s][traits]

[fmt]: /hello/print.html
[methods]: /fn/methods.html
[structs]: /custom_types/structs.html
[traits]: /trait.html
[where]: /generics/where.html
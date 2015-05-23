Similar to functions, implementations require care to remain generic.

```rust
struct S; // A null struct
struct GenericTup<T>(T,);

// impl of GenericTup we specifically specialize:
impl GenericTup<f32> {} // Specialize to `f32`
impl GenericTup<S> {} // Specialize to `S` defined above

// `<T>` Must precede the type to remain generic
impl <T> GenericTup<T> {}
```

Note: Rust does not *currently* allow overlap between implementations. The
3 separate implementations of `GenericTup` above cannot coexist. There are
[plans][specialization_plans] to fix this though.

{impl.play}

### See also:

[functions returning references][fn], [`impl`][methods], and [`struct`][structs]


[fn]: /scope/lifetime/fn.html
[methods]: /fn/methods.html
[specialization_plans]: http://blog.rust-lang.org/2015/05/11/traits.html#the-future
[structs]: /custom_types/structs.html

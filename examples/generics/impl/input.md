Similar to functions, implementations require care to remain generic.

```rust
struct S; // Concrete type `S`
struct GenericVal<T>(T,); // Generic type `GenericVal`

// impl of GenericVal where we explicitly specify type parameters:
impl GenericVal<f32> {} // Specify `f32`
impl GenericVal<S> {} // Specify `S` as defined above

// `<T>` Must precede the type to remain generic
impl <T> GenericVal<T> {}
```

{impl.play}

### See also:

[functions returning references][fn], [`impl`][methods], and [`struct`][structs]


[fn]: /scope/lifetime/fn.html
[methods]: /fn/methods.html
[specialization_plans]: http://blog.rust-lang.org/2015/05/11/traits.html#the-future
[structs]: /custom_types/structs.html

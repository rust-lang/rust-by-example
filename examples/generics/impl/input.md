Implementations can also be made generic. Generally, `impl` is followed by
the type `<Type>`, although it is not a strict requirement.

```rust
struct S; // A null struct
struct GenericTup<T>(T,);

// impl of GenericTup we specifically specialize:
impl GenericTup<f32> {} // Specialize to `f32`
impl GenericTup<S> {} // Specialize to `S` defined above

// `<T>` Must precede the type to remain generic
impl <T> GenericTup<T> {}
```

{impl.play}

###See also:
[impl](http://rustbyexample.com/methods.html),
[struct](http://rustbyexample.com/structs.html), and
[functions returning references](http://rustbyexample.com/lifetime/fn.html),


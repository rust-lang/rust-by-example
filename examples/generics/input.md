Generics is the topic of generalizing types and functionality to broader
cases. This is extremely useful in reducing code duplication in many ways,
but requires a rather involving syntax. However, we will find that being 
generic involves taking great care to specify over what types a generic type 
is actually considered valid.

A type parameter is specified as generic by the use of angle brackets and
[camel case][camelcase]: `<A, B, ...>`. "Generic type parameters" are
typically represented as `<T>`. In Rust, "generic" also describes anything that
accepts one or more generic type parameters `<T>`. Any type specified as a 
generic type parameter is generic, and everything else is concrete (non-generic).

For example, defining a *generic function* named `foo` that takes an argument
`T` of any type:

```rust
fn foo<T>(T) { ... }
```

Because `T` has been specified as a generic type parameter, it is considered
generic when used here as `(T)`. This is the case even if `T` has previously
been defined as a `struct`.

This example shows some of the syntax in action:

{generics.play}

### See also:

[`struct`s][structs]

[structs]: /custom_types/structs.html
[camelcase]: https://en.wikipedia.org/wiki/CamelCase
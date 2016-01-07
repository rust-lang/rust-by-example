Generics is the topic of generalizing types and functionality to broader
cases. This is extremely useful in reducing code duplication in many ways,
but requires a rather involving syntax. However, we will find that being 
generic involves taking great care to specify over what types a generic type 
is actually considered valid.

A type parameter is specified as generic by the use of angle brackets and
[camel case][camelcase]: `<A, B, ...>`. "Generic type parameters" are
typically represented as `<T>`. In Rust, "generic" also describes anything that
accepts one or more generic type parameters `<T>`. For example, defining a
*generic function* named `foo` that takes an argument `T` of any type:

```rust
fn foo<T>(T) { ... }
```

There are 2 basic rules regarding this which are applied at the type's first use:

* Any type previously specified to be generic is generic.
* Everything else is concrete (non-generic).

These rules play out like this:

{generics.play}

### See also:

[`struct`s][structs]

[structs]: /custom_types/structs.html
[camelcase]: https://en.wikipedia.org/wiki/CamelCase
Primitive types like `int`, `f64`, etc. and references `&'a T` are copied by
default instead of moved, because these types implement the `Copy` trait. All
the other data structures can be copied explicitly by calling the `clone()`
method, if they implement the `Clone` trait.

{clone.rs}

{clone.out}

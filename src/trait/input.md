A `trait` is a collection of method defined for an unknown type, these traits
can then be implemented for any `struct`. The implementors of a same trait can
be grouped in an array since they provide the same interface.

{trait.rs}

{trait.out}

Rust provides a special kind of traits, named *deriving* traits. These traits
implement basic functionality like equality, orderability, printability, etc.
and don't require any method to be implemented for the data, instead the
methods are implemented automatically by the compiler. The `deriving`
annotation is used to implement these traits.

{deriving.rs}

{deriving.out}

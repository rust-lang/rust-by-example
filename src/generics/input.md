Generic code is the norm in Rust, structs can be declared to hold generics
types, and functions can also be declared to take generic types as arguments.

Generics must be specialized when used, but because of type inference,
annotation is usually not required. When that's not the case, structs can be
specialized via type annotation, and functions are specialized passing the
generic arguments using this syntax `::<T, U, ..>`.

{generics.rs}

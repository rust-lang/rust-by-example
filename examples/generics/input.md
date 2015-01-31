Generic structs can be declared to hold generic types, and generic functions
can be declared to take generic types as arguments.

Generics must be specialized when used, but, because of type inference,
annotation is usually not required. When that's not the case, structs can be
specialized via type annotation, and functions can be specialized passing the
generic arguments using this syntax `::<T, U, ..>`.

{generics.play}

Rust provides type safety via its static type-checker. Variables can be type
annotated using a colon `:` after the variable name in the `let` statement,
however in most cases the compiler will be able to infer the type of a value
from the context, heavily reducing the annotation burden.

Type conversion (a.k.a. casting) must be explicitly stated using the `as`
keyword.

{type.rs}

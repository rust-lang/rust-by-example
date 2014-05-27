Rust provides type safety via its static type-checker. Variables can be type
annotated using a colon `:` after the variable name in the `let` statement,
however in most cases the compiler will be able to infer the type of the
variable from the context, heavily reducing the annotation burden.

Type conversion (a.k.a. casting) must be explicitly stated using the `as`
keyword.

{type.rs}

{type.out}

The type inference engine is pretty smart, it does more than looking at the
type of the rvalue of an initialization; it also looks how the variable is used
afterwards to infer its type. Here's an advanced example of type inference:

{inference.rs}

No type annotation of variables was needed, the compiler is happy and so is the
programmer!

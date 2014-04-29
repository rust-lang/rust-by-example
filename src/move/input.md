One of the core concepts of Rust is *ownership*, this means that variables
*own* the data they refer to. And data is **not** copied by default, instead
the ownership is transferred, this is called *moving* the data in rust-speak.

The exception to these move semantics are the primitives types like `int`,
`uint`, `f64`, etc, which are copied instead of moved since they are small in
size.

{move.rs}

{move.out}

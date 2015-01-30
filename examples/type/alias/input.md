The `type` statement can be used to give a new name to an existing type. Types
must have `CamelCase` names, or the compiler will raise a warning. The
exception to this rule are the primitive types: `usize`, `f32`, etc.

{alias.play}

The main use of aliases is to reduce typing; for example the
[`IoResult<T>`][io-result]
type is an alias for the `Result<T, IoError>` type.

[io-result]: http://doc.rust-lang.org/std/io/type.IoResult.html

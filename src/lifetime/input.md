The Rust compiler is aware of the lifetimes of objects, and uses this
information to enforce valid borrowing of objects. Lifetimes can be used as
generic arguments for structs and functions using the notation `'a`, and are
part of the type signature of references `&'a T`.

{lifetime.rs}

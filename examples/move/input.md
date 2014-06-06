Because variables are in charge of freeing their resources (if any), resources
can only have *one* owner, otherwise resources would get freed more than once.

When doing assignments, `let x = y`, or passing function arguments by value
`foo(x)`, the data is copied, but the *ownership* of the resources is
transferred, this is know as *moving* in Rust-speak.

{assignment.play}

After moving resources, the previous owner loses access to the resource. This
avoids *dereferencing freed memory*.

{pass-by-value.rs}

{pass-by-value.out}

Mutability of data depends on its owner. Mutability of data can change when
ownership is transferred.

{mut.rs}

{mut.out}

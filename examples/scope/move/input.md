Because variables are in charge of freeing their resources (if any), resources
can only have *one* owner, otherwise resources would get freed more than once.

When doing assignments `let x = y`, or passing function arguments by value
`foo(x)`, the *ownership* of the resources, if any, is transferred; this is
known as a "move" in Rust-speak.

After moving resources, the previous owner can no longer be used. This avoids
the creation of *dangling pointers*.

{move.play}

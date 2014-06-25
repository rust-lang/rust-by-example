Because variables are in charge of freeing their resources (if any), resources
can only have *one* owner, otherwise resources would get freed more than once.

When doing assignments `let x = y`, or passing function arguments by value
`foo(x)`, the data is *not* copied, but the *ownership* of the resources is
transferred, this is know as a *move* in Rust-speak.

{move.play}

After moving resources, the previous owner loses access to the resource. This
avoids the creation of *dangling pointers*.

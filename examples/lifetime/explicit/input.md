When writing functions that return references, lifetimes must be explicitly
annotated. These functions are generic and we must tell the compiler what is
the relationship between the lifetimes of the objects that appear in the
arguments and the output.

Let's illustrate with an example: we want a function that returns a reference
to the title field of a Book struct. The most generic function that we could
write would look like this:

{explicit.play}

The compiler can't tell how `'a` and `'b` are related, so we must supply this
information. The answer here is that `'a = 'b`, the reason is that the title
field will be destroyed when the book gets destroyed (same way with the
creation time), therefore the title field has the same lifetime as the book.


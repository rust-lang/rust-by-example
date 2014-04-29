All values in Rust are stack allocated by default. Values can be *boxed*
(allocated in the heap) using a tilde `~`, these boxes are actually pointers to
the values in the heap.

Boxed values can be dereferenced using the `*` operator, this means the data is
removed from the heap and brought into the stack.

{box.rs}

{box.out}

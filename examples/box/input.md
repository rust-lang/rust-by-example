All values in Rust are stack allocated by default. Values can be *boxed*
(allocated in the heap) by creating a `Box<T>`. A box is a smart pointer to a
heap allocated value of type `T`. When a box goes out of scope, its destructor
is called, the inner object is destroyed, and the memory in the heap is freed.

Boxed values can be dereferenced using the `*` operator; this removes one layer
of indirection. 

{box.play}

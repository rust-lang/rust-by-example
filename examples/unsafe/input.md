As an introduction to this section, to borrow from [the official docs](
http://doc.rust-lang.org/book/unsafe.html), "one should try to minimize the
amount of unsafe code in a code base." With that in mind, let's get started!
Unsafe blocks in Rust are used to bypass protections put in place by the
compiler; specifically, there are four primary things that unsafe blocks are
used for:

* dereferencing raw pointers
* calling a function over FFI (but this is covered in a different part of the
  book)
* changing types through `std::mem::transmute`
* inline assembly

### Raw Pointers
Raw pointers `*` and references `&T` function similarly, but references are
always safe because they are guaranteed to point to valid data due to the
borrow checker. Dereferencing a raw pointer can only be done through an unsafe
block.

{pointer.rs}

### Transmute
Allows simple conversion from one type to another, however both types must have
the same size and alignment:

{transmute.rs}

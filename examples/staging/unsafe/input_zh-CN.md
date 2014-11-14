As an introduction to this section, to borrow from [the official docs](
http://doc.rust-lang.org/master/guide-unsafe.html), "one should try to
minimize the amount of unsafe code in a code base."  With that in mind, let's
get started!
Unsafe blocks in Rust are used to bypass protections put in place by the
compiler; specifically, there are four primary things that unsafe blocks are
used for:

* dereferencing raw pointers
* calling a function over FFI (but this is covered in a different part of the
  book)
* changing types through `std::cast::transmute`
* inline assembly

### Raw Pointers
Raw pointers `*` and references `&T` function similarly, but references are
always safe because they are guaranteed to point to valid data due to the
borrow checker.  Dereferencing a raw pointer can only be done through an unsafe
block.

{pointer.rs}

### Transmute
Allows simple conversion from one type to another, however both types must have
the same size and alignment

{transmute.rs}

### Inline Assembly
Inline assembly functions very similarly to the inline assembly of c, which
makes sense considering its implementation is not handled by rust, rather by
the LLVM.  It allows for direct access to assembly manipulation, which can
massively increase speed, but it can also decrease portability and stability.
In most cases the compiler will optimize your rust code to better assembly than
you could write, so in most instances it is not worth it.  The first parameter
of asm!() is the format of the assembly, the parameter following the colon is
the output variable, and the parameter(s) following that are the input
variables.

**Note**: `#![feature(asm)]` is currently required to use inline assembly.

{asm.rs}

# Unsafe Operations

As an introduction to this section, to borrow from [the official docs](
https://doc.rust-lang.org/book/unsafe.html), "one should try to minimize the
amount of unsafe code in a code base." With that in mind, let's get started!
Unsafe blocks in Rust are used to bypass protections put in place by the
compiler; specifically, there are four primary things that unsafe blocks are
used for:

* dereferencing raw pointers
* calling a function over FFI (but this is covered in [a previous
  chapter](/std_misc/ffi.html) of the book)
* changing types through `std::mem::transmute`
* inline assembly

### Raw Pointers
Raw pointers `*` and references `&T` function similarly, but references are
always safe because they are guaranteed to point to valid data due to the
borrow checker. Dereferencing a raw pointer can only be done through an unsafe
block.

```rust,editable
fn main() {
    let raw_p: *const u32 = &10;

    unsafe {
        assert!(*raw_p == 10);
    }
}
```

### Transmute
Allows simple conversion from one type to another, however both types must have
the same size and alignment:

```rust,editable
fn main() {
    let u: &[u8] = &[49, 50, 51];

    unsafe {
        assert!(u == std::mem::transmute::<&str, &[u8]>("123"));
    }
}
```
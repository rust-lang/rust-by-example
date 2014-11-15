The compiler enforces valid borrowing using its borrow checker. To accomplish
this, it keeps track of two things:
* The *lifetime* of objects, and
* The scope of blocks

The lifetime of an object starts when the object is created and ends when it
goes out of scope (i.e. it gets destroyed, because of the RAII discipline).

A lifetime looks like this: `'burrito`, which reads as: "lifetime burrito".

All references actually have a type signature of the form `&'a T`, where
`'a` is the lifetime of the *referenced* object. The compiler takes care of
inserting the lifetime part `'a` so we can simply type annotate references with
`&T`.

For example:

``` rust
let integer: int = 5;
let ref_to_int: &int = &integer;
```

* `integer` has lifetime `'i` (it could be any other name, like `'foo`)
* `ref_to_int` has lifetime `'r` (references also have lifetimes!)
* `ref_to_int` type signature actually is `&'i int` (the compiler inserts the
  `'i` for us)
* The type signature `&'i int` reads as:
  * `&`: reference to an
  * `int`: integer with
  * `'i`: lifetime `i` (`i` is the lifetime of `integer`!)

Because the compiler keeps track of the lifetime of referenced objects in the
type system, it can avoid several memory bugs.

Haven't grokked what a lifetime is yet? Don't dismay! See the next page.

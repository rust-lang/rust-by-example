Let's see how the compiler prevents the creation of dangling pointers via its
borrow checker. To simplify the analysis and explanation, we have two
additions:

* Lifetimes has been explicitly annotated in the source code.
* We have drawn the lifetime "lines", which span from the creation of an object
  to its destruction. The block scopes have also been drawn.

Note that explicit lifetime annotation on references `&'foo T` is not allowed
by the compiler, so you must remove the lifetime part `'foo` to see the "real"
compiler error.

{borrow.play}

The "real" compiler error is: "`another_boxed_integer` does not live long
enough". Let's analyze why this happens:

* `stack_integer` has lifetime `'a`
* `boxed_integer` has lifetime `'b`
* `ref_to_box` has lifetime `'c`
* `ref_to_another_box` has lifetime `'d`
* `another_boxed_integer` has lifetime `'e`
* `'main` and `'let` are the scopes of the blocks
* When a block scope ends, all the objects declared in it get destroyed
  * `'let` ends, and so does `'e`
  * `'main` ends, and so does `'a` `'b` `'c` and `'d`
* `ref_to_box` is a valid borrow, because
  * `ref_to_box` has lifetime `'c`
  * `ref_to_box` points to an object with lifetime `'b`
  * `'c` will never *outlive* `'b` (this is expressed as `'c < 'b`)
  * therefore `ref_to_box` will always point to valid data
* `ref_to_another_box` is an *invalid* borrow, because
  * `ref_to_another_box` has lifetime `'d`
  * `ref_to_another_box` points to an object with lifetime `'e`
  * `'d` outlives `'e`
  * therefore `ref_to_another_box` can become a dangling pointer (it can point
    to destroyed data)
    * creation of dangling pointers is forbidden, so this borrow is invalid

The borrow checker will do this job for the programmer behind his/her back, to
prevent him/her from (unintentionally) creating dangling pointers. Although,
the programmer can be saved by the borrow checker without knowing what a
lifetime is.

The programmer doesn't need to explicitly annotate lifetimes (nor understand
what are lifetimes), for the borrow checker to do its job in most cases. These
are the cases where explicit lifetimes are required:

* [Functions that return references][lifetime]
* [Structs that hold references][structs]

[lifetime]: /lifetime/fn.html
[structs]: /lifetime/struct.html

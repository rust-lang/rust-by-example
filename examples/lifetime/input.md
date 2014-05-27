The compiler enforces valid borrowing using its borrow checker. To accomplish
this, it keeps track of the *lifetimes* of objects and block scopes. The
lifetime of an object starts when the object is created and ends when its
destructor is called.

The compiler annotates lifetimes with this notation: `'a`, `'b`, `'static`,
etc. All references have a type signature of the form `&'a T`, where `'a` is
the lifetime of the *referenced* object. The compiler takes care of inserting
the lifetime part `'a`, so we can simply type annotate references with `&T`.

Let's see how the compiler prevents the creation of dangling pointers via its
borrow checker. To simplify the analysis and explanation, lifetimes has been
explicitly annotated in the source code. Explicit annotation is not allowed by
the compiler in this case, so remove the lifetimes to see the "real" compiler
error.

{lifetime.rs}

**Note** The vertical bars don't look straight because the source code doesn't
get highlighted. This problem has already been
[fixed upstream](https://github.com/isagalaev/highlight.js/pull/465), and will
be amended soon.

The "real" compiler error is: "`another_boxed_integer` does not live long
enough". Let's analyze why this happens:

* `stack_integer` has lifetime `'a`
* `boxed_integer` has lifetime `'b`
* `ref_to_box` has lifetime `'c`
* `ref_to_another_box` has lifetime `'d`
* `another_boxed_integer` has lifetime `'e`
* `'main` and `'let` are the lifetimes of the block scopes
* When a block scope ends, all the objects declared in it get destroyed
  * `'let` ends, and so does `'e`
  * `'main` ends, and so does `'a` `'b` `'c` and `'d`
* `ref_to_box` is a valid borrow
  * `ref_to_box` has lifetime `'c`
  * `ref_to_box` points to an object with lifetime `'b`
  * because `'c` will never *outlive* `'b` (this is expressed as `'c < 'b`)
  * then `ref_to_box` will always point to valid data
* `ref_to_another_box` is an *invalid* borrow
  * `ref_to_another_box` has lifetime `'e`
  * `ref_to_another_box` points to an object with lifetime `'d`
  * because `'e` outlives `'d`
  * then `ref_to_another_box` can point to destroyed data (this is forbidden)

When writing functions that return references, lifetimes must be explicitly
annotated. These functions are generic and must tell the relationship between
the lifetimes of the objects in the arguments and the output.

Let's illustrate with an example: we want a function that returns a reference
to the title field of a Book struct. The most generic function that we could
write would look like this:

{reference-bad.rs}

{reference-bad.out}

The compiler can't tell how `'a` and `'b` are related, so we must supply this
information. The answer here is that `'a = 'b`, the reason is that the title
field will be destroyed when the book gets destroyed (same way with the
creation time), therefore the title field has the same lifetime as the book.

{reference-good.rs}

{reference-good.out}

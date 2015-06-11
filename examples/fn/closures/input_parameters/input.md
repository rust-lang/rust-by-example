It has been noted that Rust chooses how to capture variable on the fly
without annotation. This is all very convenient in normal usage however when
writing functions, this ambiguity is not allowed. The closure's complete
type including which capturing type must be annotated. The manner of capture
a closure uses is annotated as one of the following `traits`:

* `Fn`: takes captures by reference (`&T`)
* `FnMut`: takes captures by mutable reference (`&mut T`)
* `FnOnce`: take captures by value (`T`)

Even annotated, these are very flexible: a parameter of `FnOnce` specifies
the closure *may* capture by `T` or `&mut T` or `&T` at will (if a move is
possible, any type of borrow should also be possible). The reverse is not
true: if the parameter is `Fn`, then nothing lower is allowed. Therefore,
the rule is:

* any annotated parameter restricts capture to itself and above

In addition, Rust will preferentially capture variables in the least
restrictive manner possible on a variable-by-variable basis:

{input_parameters.play}

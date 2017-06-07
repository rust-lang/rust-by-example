While Rust chooses how to capture variables on the fly mostly without type 
annotation, this ambiguity is not allowed when writing functions. When 
taking a closure as an input parameter, the closure's complete type must be 
annotated using one of a few `traits`. In order of decreasing restriction, 
they are:

* `Fn`: the closure captures by reference (`&T`)
* `FnMut`: the closure captures by mutable reference (`&mut T`)
* `FnOnce`: the closure captures by value (`T`)

On a variable-by-variable basis, the compiler will capture variables in the 
least restrictive manner possible. 

For instance, consider a parameter annotated as `FnOnce`. This specifies 
that the closure *may* capture by `&T`, `&mut T`, or `T`, but the compiler 
will ultimately choose based on how the captured variables are used in the 
closure.

This is because if a move is possible, then any type of borrow should also 
be possible. Note that the reverse is not true. If the parameter is 
annotated as `Fn`, then capturing variables by `&mut T` or `T` are not 
allowed.

In the following example, try swapping the usage of `Fn`, `FnMut`, and 
`FnOnce` to see what happens:

{input_parameters.play}

### See also:

[`std::mem::drop`][drop], [`Fn`][fn], [`FnMut`][fnmut], and [`FnOnce`][fnonce]

[drop]: https://doc.rust-lang.org/std/mem/fn.drop.html
[fn]: https://doc.rust-lang.org/std/ops/trait.Fn.html
[fnmut]: https://doc.rust-lang.org/std/ops/trait.FnMut.html
[fnonce]: https://doc.rust-lang.org/std/ops/trait.FnOnce.html

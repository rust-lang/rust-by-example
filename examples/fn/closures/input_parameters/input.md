While Rust chooses how to capture variables on the fly mostly without 
annotation, this ambiguity is not allowed when writing functions. When 
taking a closure as an input parameter, Rust will preferentially capture 
variables in the least restrictive manner possible on a variable-by-variable 
basis.

A closure's complete type must be annotated, including the captured type, as 
one of the following `traits`:

* `Fn`: the closure takes capture by reference (`&T`)
* `FnMut`: the closure takes capture by mutable reference (`&mut T`)
* `FnOnce`: the closure takes capture by value (`T`)

Even annotated, these are very flexible: a parameter of `FnOnce` specifies
the closure *may* capture by `T`, `&mut T`, or `&T` at will. This is because  
if a move is possible, then any type of borrow should also be possible. The 
reverse is not true: if the parameter is `Fn`, then nothing lower on the 
list is allowed. 

In the following example, try swapping the usage of `Fn`, `FnMut`, and 
`FnOnce` to see what happens:

{input_parameters.play}

### See also:

[`std::mem::drop`][drop], [`Fn`][fn], [`FnMut`][fnmut], and [`FnOnce`][fnonce]

[drop]: http://doc.rust-lang.org/std/mem/fn.drop.html
[fn]: http://doc.rust-lang.org/std/ops/trait.Fn.html
[fnmut]: http://doc.rust-lang.org/std/ops/trait.FnMut.html
[fnonce]: http://doc.rust-lang.org/std/ops/trait.FnOnce.html

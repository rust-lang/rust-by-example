Since closures may be used as arguments, you might wonder if the same can be said 
about functions. And indeed they can! However, because a function can 
*never* capture variables, closures are strictly more flexible. Therefore, any 
function which can take a closure as an argument can also take a function.

{input_functions.play}

As an additional note, the `Fn`, `FnMut`, and `FnOnce` `traits` dictate how
a closure captures variables from the enclosing scope. 

### See also:

[`Fn`][fn], [`FnMut`][fn_mut], and [`FnOnce`][fn_once]

[fn]: http://doc.rust-lang.org/std/ops/trait.Fn.html
[fn_mut]: http://doc.rust-lang.org/std/ops/trait.FnMut.html
[fn_once]: http://doc.rust-lang.org/std/ops/trait.FnOnce.html

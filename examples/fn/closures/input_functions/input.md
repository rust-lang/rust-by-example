Since closures are possible as arguments, you might wonder if functions
are possible as well. Indeed they are! The previously mentioned `Fn`,
`FnMut`, and `FnOnce` `traits` all dictate in what fashion a closure captures
variables from the enclosing scope. Because a function can *never* capture
variables, closures are strictly more flexible. Therefore, any function which 
can take a closure as an argument can also take a function.

{input_functions.play}

### See also:

[`Fn`][fn], [`FnMut`][fn_mut], and [`FnOnce`][fn_once]

[fn]: http://doc.rust-lang.org/std/ops/trait.Fn.html
[fn_mut]: http://doc.rust-lang.org/std/ops/trait.FnMut.html
[fn_once]: http://doc.rust-lang.org/std/ops/trait.FnOnce.html

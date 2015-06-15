Since closures are possible as arguments, you might wonder if functions
are also possible and indeed they are. The previously mentioned `Fn`,
`FnMut`, and `FnOnce` `traits` all dictate what fashion a closure captures
variables from the enclosing scope. A function can *never* capture variables
and thus is strictly less flexible. Therefore, any function which can
take a closure as an argument can also take a function.

{input_functions.play}

### See also:

[`Fn`][fn], [`FnMut`][fn_mut], and [`FnOnce`][fn_once]

[fn]: http://doc.rust-lang.org/std/ops/trait.Fn.html
[fn_mut]: http://doc.rust-lang.org/std/ops/trait.FnMut.html
[fn_once]: http://doc.rust-lang.org/std/ops/trait.FnOnce.html

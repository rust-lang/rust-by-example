Since closures may be used as arguments, you might wonder if the same can be said
about functions. And indeed they can! If you declare a function that takes a
closure as parameter, then any function that satisfies the trait bound of that
closure can be passed as a parameter.

{input_functions.play}

As an additional note, the `Fn`, `FnMut`, and `FnOnce` `traits` dictate how
a closure captures variables from the enclosing scope.

### Смотрите также:

[`Fn`][fn], [`FnMut`][fn_mut], and [`FnOnce`][fn_once]

[fn]: http://doc.rust-lang.org/std/ops/trait.Fn.html
[fn_mut]: http://doc.rust-lang.org/std/ops/trait.FnMut.html
[fn_once]: http://doc.rust-lang.org/std/ops/trait.FnOnce.html

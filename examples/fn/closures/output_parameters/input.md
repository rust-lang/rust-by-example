Closures as input parameters are possible so returning one should also be
possible. However, returning closure types are problematic because Rust
currently only supports returning concrete (non-generic) types. Anonymous
closure types are, by definition, unknown and so returning a closure is only
possible by making it concrete. This can be done via boxing.

The valid types for returns are slightly different than before:

* `Fn`: normal
* `FnMut`: normal
* `FnBox`: equivalent to `FnOnce` but specialized for this application
because `FnOnce` currently interacts badly with the type system.

Beyond this, the `move` keyword must be used which signals that all captures
occur by value. This is required because any captures by reference would be
dropped as soon as the function exited leaving invalid references in the
closure.

{output_parameters.play}

### See also:

[Boxing][box], [`Fn`][fn], [`FnMut`][fnmut], [`FnBox`][fnbox], and
[Generics][generics]

[box]: /std/box.html
[fn]: http://doc.rust-lang.org/std/ops/trait.Fn.html
[fnmut]: http://doc.rust-lang.org/std/ops/trait.FnMut.html
[fnbox]: http://doc.rust-lang.org/std/boxed/trait.FnBox.html
[generics]: /generics.html

Using closures as input parameters are possible, so returning closures as 
output parameters should also be possible. However, returning closure types 
are problematic because Rust currently only supports returning concrete 
(non-generic) types. Anonymous closure types are, by definition, unknown 
and so returning a closure is only possible by making it concrete. This 
can be done via boxing.

The valid traits for returns are slightly different than before:

* `Fn`: normal
* `FnMut`: normal
* `FnOnce`: There are some unusual things at play here, so the [`FnBox`][fnbox]
  type is currently needed, and is unstable. This is expected to change in
  the future.

Beyond this, the `move` keyword must be used, which signals that all captures
occur by value. This is required because any captures by reference would be
dropped as soon as the function exited, leaving invalid references in the
closure.

{output_parameters.play}

### See also:

[Boxing][box], [`Fn`][fn], [`FnMut`][fnmut], and [Generics][generics].

[box]: /std/box.html
[fn]: http://doc.rust-lang.org/std/ops/trait.Fn.html
[fnmut]: http://doc.rust-lang.org/std/ops/trait.FnMut.html
[fnbox]: http://doc.rust-lang.org/std/boxed/trait.FnBox.html 
[generics]: /generics.html

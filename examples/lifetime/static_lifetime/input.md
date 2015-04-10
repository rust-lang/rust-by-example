A `'static` lifetime is one which lasts for the lifetime of the running
program. There are two ways to make a variable with `'static` lifetime:

* Make a `"string"` literal which has type: `&'static str`.
* Make a constant with the `static` declaration.

{static_lifetime.play}

### See also:

[`'static` constants][static_const]

[static_const]: ./constants.html

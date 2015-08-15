A `'static` lifetime is one which lasts for the lifetime of the running
program. There are two ways to make a variable with `'static` lifetime:

* Make a `"string"` literal which has type: `&'static str`.
* Make a constant with the `static` declaration.

`statics` may also be coerced to smaller lifetimes since they are clearly
larger than most:

{static_lifetime.play}

### See also:

[`'static` constants][static_const]

[static_const]: /custom_types/constants.html

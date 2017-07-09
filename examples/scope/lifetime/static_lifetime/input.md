A `'static` lifetime is the longest possible lifetime, and lasts for 
the lifetime of the running program. A `'static` lifetime may also be 
coerced to a shorter lifetime. There are two ways to make a variable 
with `'static` lifetime, and both are stored in the read-only memory
of the binary:

* Make a constant with the `static` declaration.
* Make a `string` literal which has type: `&'static str`.

See the following example for a display of each method:

{static_lifetime.play}

### Смотрите также:

[`'static` constants][static_const]

[static_const]: ../../custom_types/constants.html

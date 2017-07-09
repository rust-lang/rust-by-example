Ignoring [elision], function signatures with lifetimes have a few constraints:

* any reference *must* have an annotated lifetime.
* any reference being returned *must* have the same lifetime as an input or
be `static`.

Additionally, note that returning references without input is banned if it
would result in returning references to invalid data. The following example shows
off some valid forms of functions with lifetimes:

{fn.play}

### Смотрите также:

[functions][fn]

[elision]: ../../scope/lifetime/elision.html
[fn]: ../../fn.html

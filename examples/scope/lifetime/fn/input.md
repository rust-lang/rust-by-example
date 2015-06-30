Functions with lifetimes have a few different valid forms. Ignoring
[elision][elision] for the time being, the rules for function parameters are:

* any reference *must* have an annotated lifetime.
* any reference being returned *must* have the same lifetime as an input.

{fn.play}

### See also:

[functions][fn]


[elision]: http://doc.rust-lang.org/nightly/book/lifetimes.html#lifetime-elision
[fn]: /fn.html

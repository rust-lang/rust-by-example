A `trait` is a collection of methods declared/defined for an unknown type:
`Self`. Traits can be implemented for any data type.

**Note**: we are using `&'static str` below to simplify ownership semantics for
this particular example. In general, it is more practical to use `&str` or
`String` (an owned string type).

{trait.play}

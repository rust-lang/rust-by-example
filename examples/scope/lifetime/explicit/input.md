The borrow checker utilizes explicit lifetime annotations to reason about
how long references should be valid. Failure to annotate lifetimes[^1] is akin
to banning the borrow checker from validating borrows and so accordingly,
annotation is mandatory.

Since lifetimes *currently* have no explicit type or name associated with them,
usage will require generics (similar to [closures][anonymity]). Additionally,
a second meaning will be associated with this lifetime syntax. `foo<'a, 'b>`
states:

1. `'a` and `'b` will represent names for lifetimes with non-specifiable
(generic) types.
2. The lifetime of `foo` may not exceed either lifetimes `'a` or `'b`.

Explicit annotation of a type has the form: `&'a T` where `'a` has already
been introduced. The less obvious of the rules is **Rule 2** whose
importance can usually be directly deduced from borrowing rules. Consider the
following example:

{explicit.play}

[^1]: [elision][elision] implicitly annotates lifetimes and so is different.

### See also:

[generics][generics] and [closures][closures]


[anonymity]: /fn/closures/anonymity.html
[closures]: /fn/closures.html
[elision]: /scope/lifetime/elision.html
[generics]: /generics.html

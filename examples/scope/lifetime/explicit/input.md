The borrow checker utilizes explicit lifetime annotation to reason about
how long references should be valid. Failure to annotate lifetimes[^1] is akin
to banning the borrow checker from validating borrows and so accordingly,
annotation is mandatory.

Since lifetimes *currently* have no explicit type or name associated with them,
usage will require generics (similar to [closures][anonymity]). Somewhat
peculiarly, lifetime annotation has a second additional meaning. `foo<'a, 'b>`
states:

1. `'a` and `'b` will represent names for lifetimes with non-specifiable
(generic) types.
2. The lifetime of `foo` may not exceed either lifetimes `'a` or `'b`.

Explicit annotation of a type has the form: `&'a T` where `'a` has already
been introduced. Together with **Rule 2**, this specifies that any borrow
*must* eventually be returned. That is, if a function/type borrows a
reference, when the borrower ceases, the reference must be returned *or* the
borrow would be invalid! Borrowing and never returning cannot really be called
borrowing.

{explicit.play}

[^1]: [elision][elision] implicitly annotates lifetimes and so is different.

### See also:

[generics][generics] and [closures][closures]


[anonymity]: /fn/closures/anonymity.html
[closures]: /fn/closures.html
[elision]: /scope/lifetime/elision.html
[generics]: /generics.html

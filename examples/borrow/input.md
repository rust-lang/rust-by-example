Most of times, we'll like to access some data, without taking ownership over
it. To accomplish this, Rust provides a *borrowing* mechanism. Instead of
passing objects by-value (`T`), objects can be passed by reference (`&T`). The
compiler statically guarantees that references *always* point to valid objects,
via its borrow checker.

{borrow.rs}

{borrow.play}

{borrow.out}

`&T` borrows the data via an immutable reference, and the borrower can read the
data but not modify it. Mutable data can be immutably borrowed, but can also be
mutably borrowed via a mutable reference `&mut T`, giving read/write access to
the borrower.

{mut.rs}

{mut.play}

{mut.out}

When data is borrowed, it also *freezes*. "Frozen" data can't be modified via
the original object, until all the references to it go out of scope.

{freeze.rs}

{freeze.play}

{freeze.out}

Data can be immutably borrowed any number of times, but once immutably
borrowed, the original data can't be mutably borrowed. On the other side, data
can be mutably borrowed *once*, and until the mutable reference goes out of
scope, the original data can't be borrowed again.

{re-borrow.rs}

{re-borrow.play}

{re-borrow.out}

When doing pattern matching or destructuring via the `let` binding, the `ref`
keyword can be used to take references to the fields of a struct.

{ref.rs}

{ref.play}

{ref.out}

Most of the time, we'd like to access some data without taking ownership over
it. To accomplish this, Rust provides a *borrowing* mechanism. Instead of
passing objects by-value (`T`), objects can be passed by reference (`&T`).

{borrow.play}

The compiler statically guarantees that references *always* point to valid
objects, via its borrow checker. For example, the original object can't be
destroyed, while references to it exists.

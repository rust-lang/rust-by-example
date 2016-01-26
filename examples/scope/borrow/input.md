Most of the time, we'd like to access data without taking ownership over
it. To accomplish this, Rust uses a *borrowing* mechanism. Instead of
passing objects by-value (`T`), objects can be passed by reference (`&T`).

The compiler statically guarantees (via its borrow checker) that references 
*always* point to valid objects. That is, while references to an object
exist, the object cannot be destroyed.

{borrow.play}

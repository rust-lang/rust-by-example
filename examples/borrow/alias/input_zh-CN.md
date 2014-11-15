Data can be immutably borrowed any number of times, but while immutably
borrowed, the original data can't be mutably borrowed. On the other side,
only *one* mutable borrow is allowed at a time. The original data can be
borrowed again after the mutable reference goes out of scope.

{alias.play}

Data can be immutably borrowed any number of times, but while immutably
borrowed, the original data can't be mutably borrowed. On the other hand,
only *one* mutable borrow is allowed at a time. The original data can be
borrowed again only *after* the mutable reference goes out of scope.

{alias.play}

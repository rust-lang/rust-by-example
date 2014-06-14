Data can be immutably borrowed any number of times, but once immutably
borrowed, the original data can't be mutably borrowed. On the other side, data
can be mutably borrowed *once*, and until the mutable reference goes out of
scope, the original data can't be borrowed again.

{alias.play}

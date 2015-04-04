Mutability of data in a Box<t> can be changed when ownership is transferred.

However, once an immutable reference to an object is borrowed, the reference
cannot be made mutable by transferring ownership. This strict "const
correctness" is one of the reasons to prefer references over Box<T>'s whenever
possible.

{mut.play}

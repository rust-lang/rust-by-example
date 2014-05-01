Vectors are re-sizable arrays, like slices their size is not known at compile
time, and they can be grow or shrink at anytime. A vector is represented using
3 words: a pointer to the data, its length and its capacity. The capacity
indicates how much memory is reserved for the vector, the vector can grow as
long as the length is smaller than the capacity, when this threshold needs to
be surpassed, the vector gets reallocated with a bigger capacity.

{vec.rs}

{vec.out}

More `Vec` methods can be found under the
[std::vec](http://static.rust-lang.org/doc/master/std/vec/index.html) module

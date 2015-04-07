Vectors are re-sizable arrays. Like slices, their size is not known at compile
time, but they can grow or shrink at any time. A vector is represented using
3 words: a pointer to the data, its length, and its capacity. The capacity
indicates how much memory is reserved for the vector. The vector can grow as
long as the length is smaller than the capacity. When this threshold needs to
be surpassed, the vector is reallocated with a larger capacity.

{vec.play}

More `Vec` methods can be found under the
[std::vec][vec] module

[vec]: http://doc.rust-lang.org/std/vec/

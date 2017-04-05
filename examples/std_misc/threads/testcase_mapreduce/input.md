Rust makes it very easy to parallelise data processing, without many of the headaches traditionally associated with such an attempt.

The standard library provides great threading primitives out of the box.
These, combined with Rust's concept of Ownership and aliasing rules, automatically prevent data races.

The aliasing rules (one writable reference XOR many readable references) automatically prevent you from
manipulating state that is visible to other threads. (Where synchronisation is needed, there are synchronisation
primitives like `Mutex`es or `Channel`s.)

In this example, we will calculate the sum of all digits in a block of numbers.
We will do this by parcelling out chunks of the block into different threads. Each thread will sum its tiny block of digits, and subsequently we will sum the intermediate sums produced by each thread.
Note that, although we're passing references across thread boundaries, Rust understands that we're only passing read-only references, and that thus no unsafety or data races can occur. Because we're `move`-ing the data segments into the thread, Rust will also ensure the data is kept alive until the threads exit, so no dangling pointers occur.

{mapreduce.play}

### See also:
* [Threads][thread]
* [vectors][vectors] and [iterators][iterators]
* [closures][closures], [move][move] semantics and [`move` closures][move_closure]
* [destructuring][destructuring] assignments
* [turbofish notation][turbofish] to help type inference
* [unwrap vs. expect][unwrap]
* [enumerate][enumerate]

[thread]: /std_misc/threads.html
[vectors]: /std/vec.html
[iterators]: /trait/iter.html
[destructuring]: https://doc.rust-lang.org/book/patterns.html#destructuring
[closures]: /fn/closures.html
[move]: /scope/move.html
[move_closure]: https://doc.rust-lang.org/book/closures.html#move-closures
[turbofish]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect
[unwrap]: /error/option_unwrap.html
[enumerate]: https://doc.rust-lang.org/book/loops.html#enumerate

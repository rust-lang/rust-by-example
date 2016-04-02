The `Iterator` trait is used to implement iterators over collections such as arrays.

The trait requires only a method to be defined for the `next` element, 
which may be manually defined in an `impl` block or automatically 
defined (as in arrays and ranges).

As a point of convenience for common situations, the `for` construct 
turns some collections into iterators using the [`.into_iterator()`][intoiter] method.

Methods that can be accessed using the `Iterator` trait in addition 
to those shown in the example below can be found [here][iter].

{iter.play}

[intoiter]: https://doc.rust-lang.org/std/iter/trait.IntoIterator.html
[iter]: http://doc.rust-lang.org/core/iter/trait.Iterator.html

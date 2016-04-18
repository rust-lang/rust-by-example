`Iterator::find` is a function which when passed an iterator, will return
the first element which satisfies the predicate as an `Option`. Its
signature:

```rust
pub trait Iterator {
    // The type being iterated over.
    type Item;

    // `find` takes `&mut self` meaning the caller may be borrowed
    // and modified, but not consumed.
    fn find<P>(&mut self, predicate: P) -> Option<Self::Item> where
        // `FnMut` meaning any captured variable may at most be
        // modified, not consumed. `&Self::Item` states it takes
        // arguments to the closure by reference.
        P: FnMut(&Self::Item) -> bool {}
}
```

{iter_find.play}

### See also:

[`std::iter::Iterator::find`][find]

[find]: http://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find

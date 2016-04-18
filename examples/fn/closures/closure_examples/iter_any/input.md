`Iterator::any` is a function which when passed an iterator, will return
`true` if any element satisfies the predicate. Otherwise `false`. Its
signature:

```rust
pub trait Iterator {
    // The type being iterated over.
    type Item;

    // `any` takes `&mut self` meaning the caller may be borrowed
    // and modified, but not consumed.
    fn any<F>(&mut self, f: F) -> bool where
        // `FnMut` meaning any captured variable may at most be
        // modified, not consumed. `Self::Item` states it takes
        // arguments to the closure by value.
        F: FnMut(Self::Item) -> bool {}
}
```

{iter_any.play}

### See also:

[`std::iter::Iterator::any`][any]

[any]: http://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any

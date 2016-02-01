Closures succinctly capture variables from enclosing scopes. Does this have
any consequences? It surely does. Observe how using a closure in a function
requires [generics], which is necessary because of how they are defined:

```rust
// `F` must be generic.
fn apply<F>(f: F) where
    F: FnOnce() {
    f()
}
```

When a closure is defined, the compiler implicitly creates a new
anonymous structure to store the captured variables inside, meanwhile
implementing the functionality via one of the `traits`: `Fn`, `FnMut`, or
`FnOnce` for this unknown type. This type is assigned to the variable which
is stored until calling.

Since this new type is of unknown type, any usage in a function will require
generics. However, an unbounded type parameter `<T>` would still be ambiguous
and not be allowed. Thus, bounding by one of the `traits`: `Fn`, `FnMut`, or
`FnOnce` (which it implements) is sufficient to specify its type.

{anonymity.play}

### See also:

[A thorough analysis][thorough_analysis], [`Fn`][fn], [`FnMut`][fn_mut],
and [`FnOnce`][fn_once]

[generics]: /generics.html
[fn]: http://doc.rust-lang.org/std/ops/trait.Fn.html
[fn_mut]: http://doc.rust-lang.org/std/ops/trait.FnMut.html
[fn_once]: http://doc.rust-lang.org/std/ops/trait.FnOnce.html
[thorough_analysis]: http://huonw.github.io/blog/2015/05/finding-closure-in-rust/

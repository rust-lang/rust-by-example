The borrow checker uses explicit lifetime annotations to determine
how long references should be valid. In cases where lifetimes are not
elided[^1], Rust requires explicit annotations to determine what the
lifetime of a reference should be. The syntax for explicitly annotating
a lifetime uses an apostrophe character as follows:

```rust
foo<'a>
// `foo` has a lifetime parameter `'a`
```

Similar to [closures][anonymity], using lifetimes requires generics.
Additionally, this lifetime syntax indicates that the lifetime of `foo`
may not exceed that of `'a`. Explicit annotation of a type has the form
`&'a T` where `'a` has already been introduced.

In cases with multiple lifetimes, the syntax is similar:

```rust
foo<'a, 'b>
// `foo` has lifetime parameters `'a` and `'b`
```

In this case, the lifetime of `foo` cannot exceed that of either `'a` *or* `'b`.

See the following example for explicit lifetime annotation in use:

{explicit.play}

[^1]: [elision][elision] implicitly annotates lifetimes and so is different.

### Смотрите также:

[generics][generics] and [closures][closures]

[anonymity]: ../../fn/closures/anonymity.html
[closures]: ../../fn/closures.html
[elision]: ../../scope/lifetime/elision.html
[generics]: ../../generics.html

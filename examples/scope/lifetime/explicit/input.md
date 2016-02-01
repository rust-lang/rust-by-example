The borrow checker uses explicit lifetime annotations to determine
how long references should be valid. In cases where lifetimes are not
elided[^1], failure to annotate lifetimes is akin to banning the borrow 
checker from validating borrows and annotation is mandatory. 
The syntax for explicitly annotating a lifetime uses an apostrophe 
character as follows: 

```rust
foo<'a>
// `foo` has a lifetime parameter `'a`
```

Usage of lifetimes requires generics (similar to [closures][anonymity]) 
since lifetimes have no type or name associated with them. Additionally, 
this lifetime syntax indicates that the lifetime of `foo` may not exceed 
that of `'a`. Explicit annotation of a type has the form `&'a T` where 
`'a` has already been introduced.

In cases with multiple lifetimes, the syntax is similar:

```rust
foo<'a, 'b>
// `foo` has lifetime parameters `'a` and `'b`
```

In this case, the lifetime of `foo` cannot exceed that of either `'a` *or* `'b`.

See the following example for explicit lifetime annotation in use:

{explicit.play}

[^1]: [elision][elision] implicitly annotates lifetimes and so is different.

### See also:

[generics][generics] and [closures][closures]

[anonymity]: /fn/closures/anonymity.html
[closures]: /fn/closures.html
[elision]: /scope/lifetime/elision.html
[generics]: /generics.html

A `where` clause is an alternate way to express a bound. It is done by
expressing the bound immediately before the opening `{` instead of at the
types first mention. Some reasons a `where` is useful include:

* It is clearer to specify the generic types and bounds separately than
together. For example, these two are equivalent:

```rust
impl <A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}
impl <A, D> MyTrait<A, D> for YourType where
    A: TraitB + TraitC,
    D: TraitE + TraitF {}
```

* `where` clauses are more expressive than the normal syntax. They can
apply bounds to arbitrary types rather than just type parameters. The
following example cannot be directly expressed without a `where` clause:

{where.play}

### See also:

[RFC][where], [`struct`s][structs], and [`trait`s][traits]

[structs]: /custom_types/structs.html
[traits]: /trait.html
[where]: https://github.com/rust-lang/rfcs/blob/master/text/0135-where.md

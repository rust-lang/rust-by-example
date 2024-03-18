# Where clauses

A bound can also be expressed using a `where` clause immediately before the
opening `{`, rather than at the type's first mention. Additionally, `where`
clauses can apply bounds to arbitrary types, rather than just to type
parameters.

Some cases that a `where` clause is useful:

- When specifying generic types and bounds separately is clearer:

```rust,ignore
impl<A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}

// Expressing bounds with a `where` clause
impl<A, D> MyTrait<A, D> for YourType
where
    A: TraitB + TraitC,
    D: TraitE + TraitF,
{
}
```

- When using a `where` clause is more expressive than using normal syntax. The
  `impl` in this example cannot be directly expressed without a `where` clause:

```rust,editable
use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}

// Because we would otherwise have to express this as `T: Debug` or
// use another method of indirect approach, this requires a `where` clause:
impl<T> PrintInOption for T
where
    Option<T>: Debug,
{
    // We want `Option<T>: Debug` as our bound because that is what's
    // being printed. Doing otherwise would be using the wrong bound.
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

fn main() {
    let vec = vec![1, 2, 3];

    vec.print_in_option();
}
```

### See also:

[RFC][where], [`struct`][struct], and [`trait`][trait]

[struct]: ../custom_types/structs.md
[trait]: ../trait.md
[where]: https://github.com/rust-lang/rfcs/blob/master/text/0135-where.md

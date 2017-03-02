The use of "Associated types" improves the overall readability of code
by moving inner types locally into a trait as *output* types. Syntax
for the `trait` definition is as follows:

```rust
// `A` and `B` are defined in the trait via the `type` keyword.
// (Note: `type` in this context is different from `type` when used for
// aliases).
trait Contains {
    type A;
    type B;

    // Updated syntax to refer to these new types generically.
    fn contains(&self, &Self::A, &Self::B) -> bool;
}
```

Note that functions that use the `trait` `Contains` are no longer required
to express `A` or `B` at all:

```rust
// Without using associated types
fn difference<A, B, C>(container: &C) -> i32 where
    C: Contains<A, B> { ... }

// Using associated types
fn difference<C: Contains>(container: &C) -> i32 { ... }
```

Let's rewrite the example from the previous section using associated types:

{types.play}

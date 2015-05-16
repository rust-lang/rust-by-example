Associated types forms the solution by moving the internal types locally into
the trait to be called *output* types. This shields future users of the trait
from the substantial boilerplate previously required. For example:

```rust
// The original trait
trait Contains<A, B> {
    fn contains(&self, &A, &B) -> bool;
}

// Updated. Now `A` and `B` are defined in the trait via the `type` keyword
// (Note: `type` is this context is different from `type` when used for
// aliases).
trait Contains {
    type A;
    type B;

    // Updated syntax to refer to these new types generically.
    fn contains(&self, &Self::A, &Self::B) -> bool;
}
```

Users of the `trait` reap the greatest benefits though. This generic function
doesn't need to express `A` or `B` at all:

```rust
// The original
fn difference<A, B, C>(container: &C) -> i32 where
    C: Contains<A, B> { ... }

// The updated now no longer has `A` and `B` as requirements.
fn difference<C: Contains>(container: &C) -> i32 { ... }
```

{types.play}

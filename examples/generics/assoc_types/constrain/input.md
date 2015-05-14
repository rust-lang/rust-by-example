Functionality depending on associated types may require applying bounds.

```rust
trait Contains {
    type A;
    ...
}

// Bounds are applied through the container: `C::A: Trait`.
fn apply_bounds<C>(c: C) where
    C: Contains,
    C::A: Display { ... }

// Caveat: This hasn't been implemented yet and doesn't work.
//
// Equality is also through the container: `C::A = type`.
fn apply_eq<C>(c: C) where
    C: Contains,
    C::A = i32 { ... }

// Note: There is no bounds shorthand. Only equality.
//
// An equality shorthand is provided:
fn apply_eq_shorthand<C>(c: C) where
    C: Contains<A = i32> { ... }
```

The full example:

{constrain.play}

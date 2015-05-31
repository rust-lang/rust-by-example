A useful method of unit conversions can be examined by implementing `Add`
for a phantom type. The `Add` `trait` is examined below:

```rust
// This construction would impose: `Self + RHS = Output`.
// where RHS and Self are of the same type.
pub trait Add<RHS = Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}

// `Output` must be `T` so that `T + T = T`.
impl Add for T {
    type Output = T;
    ...
}

// Similarly, this imposes: `T<U> + T<U> = T<U>`
impl<U> Add for T<U> {
    type Output = T<U>;
    ...
}
```

The whole implementation:

{units.play}

### See also:

[Borrowing (`&`)](/scope/borrow.html),
[Bounds (`X: Y`)](/trait/bounds.html),
[enum](/custom_types/enum.html),
[impl & self](/fn/methods.html),
[Overloading](/trait/ops.html),
[ref](/scope/borrow/ref.html),
[Traits (`X for Y`)](/trait.html), and
[TupleStructs](/custom_types/structs.html).


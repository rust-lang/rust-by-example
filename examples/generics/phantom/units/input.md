A useful method of unit conversions can be examined by implementing `Add`
for a phantom type. The `Add` `trait` is examined below:

```rust
// This construction would impose: `Self + RHS = Output`.
pub trait Add<RHS = Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}

// So, `Output` must be `T` and therefore, `T + T = T`.
impl<T> Add<T, Output = T> for T {}

// Similarly, this imposes: `S<T> + S<T> = S<T>` can be
// added only when `T + T = T`.
impl<S<T>> Add<S<T> for S<T> where
    T: Add<T, Output = T> {
    type Output = S<T>;
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


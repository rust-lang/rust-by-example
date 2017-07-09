A useful method of unit conversions can be examined by implementing `Add`
with a phantom type parameter. The `Add` `trait` is examined below:

```rust
// This construction would impose: `Self + RHS = Output`
// where RHS defaults to Self if not specified in the implementation.
pub trait Add<RHS = Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}

// `Output` must be `T<U>` so that `T<U> + T<U> = T<U>`.
impl<U> Add for T<U> {
    type Output = T<U>;
    ...
}
```

The whole implementation:

{units.play}

### Смотрите также:

[Borrowing (`&`)], [Bounds (`X: Y`)], [enum], [impl & self],
[Overloading], [ref], [Traits (`X for Y`)], and [TupleStructs].

[Borrowing (`&`)]: ../../scope/borrow.html
[Bounds (`X: Y`)]: ../../generics/bounds.html
[enum]: ../../custom_types/enum.html
[impl & self]: ../../fn/methods.html
[Overloading]: ../../trait/ops.html
[ref]: ../../scope/borrow/ref.html
[Traits (`X for Y`)]: ../../trait.html
[TupleStructs]: ../../custom_types/structs.html
[std::marker::PhantomData]: https://doc.rust-lang.org/std/marker/struct.PhantomData.html

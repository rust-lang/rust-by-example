# Inference

The type inference engine is pretty smart. It does more than looking at the type
of the value expression during an initialization. It also looks at how the
variable is used afterwards to infer its type. Here's an advanced example of
type inference:

```rust,editable
fn main() {
    // Because of the annotation, the compiler knows that `elem` has type u8.
    let elem = 5u8;

    // Create an empty vector (a growable array).
    let mut vec = Vec::new();
    // At this point the compiler doesn't know the exact type of `vec`, it
    // just knows that it's a vector of something (`Vec<_>`).

    // Insert `elem` in the vector.
    vec.push(elem);
    // Aha! Now the compiler knows that `vec` is a vector of `u8`s (`Vec<u8>`)
    // TODO ^ Try commenting out the `vec.push(elem)` line

    println!("{:?}", vec);
}
```

No type annotation of variables was needed, the compiler is happy and so is the
programmer!

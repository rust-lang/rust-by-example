# Const generics

Types and functions can be parameterized by constant values — most
often array lengths. `struct Matrix<const N: usize>` is a different
type for each `N`, and `fn sum<const N: usize>(a: [i32; N])` accepts an
array of any length, all checked at compile time.

```rust,editable
use std::fmt::Debug;

#[derive(Debug)]
struct Matrix<const N: usize> {
    rows: [[f64; N]; N],
}

impl<const N: usize> Matrix<N> {
    fn zeros() -> Self {
        Matrix { rows: [[0.0; N]; N] }
    }
}

fn sum<const N: usize>(values: [i32; N]) -> i32 {
    values.iter().sum()
}

fn main() {
    let m = Matrix::<2>::zeros();
    println!("{:?}", m);
    println!("sum3: {}", sum([1, 2, 3]));
    println!("sum5: {}", sum([1, 2, 3, 4, 5]));
}
```

Const generics are still more limited than type generics: expressions
involving generic constants (like `[0; N + 1]`) are not allowed on
stable, and most trait bounds on const parameters are unstable. When
you hit those walls, a macro or a `Vec` is the pragmatic fallback.

### See also:

[Const Generics in the Reference][ref] and [Arrays][arrays].

[ref]: https://doc.rust-lang.org/reference/items/generics.html#const-generics
[arrays]: ../primitives/array.md

### Exercise: Generalize a dot product

Task: Generalize this three-element dot product to arrays of any length.

<details><summary>Hint</summary>

The length can become a parameter of the function, letting the two array arguments share it.

</details>

<details><summary>Solution</summary>

```rust,editable
fn dot<const N: usize>(a: [i32; N], b: [i32; N]) -> i32 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

fn main() {
    assert_eq!(dot([1, 2, 3], [4, 5, 6]), 32);
    assert_eq!(dot([1, 2], [3, 4]), 11);
    println!("dot products check out");
}
```
</details>

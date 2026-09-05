# Iterators in depth

Iterator *adapters* (`map`, `filter`, `take`, `zip`) transform lazily and
return a new iterator, while *consumers* (`collect`, `sum`, `fold`,
`for_each`) drive the iterator to completion and produce a final value.
Nothing runs until a consumer pulls: adapters without a consumer do no
work at all.

```rust,editable
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];

    // Adapters chain lazily; `collect` consumes the chain into a Vec.
    let squares: Vec<_> = numbers.iter().map(|n| n * n).collect();
    println!("squares: {:?}", squares);

    // `fold` consumes with an accumulator instead of allocating.
    let total: i32 = numbers.iter().filter(|n| *n % 2 == 0).sum();
    println!("even sum: {total}");

    let product = numbers.iter().fold(1, |acc, n| acc * n);
    println!("product: {product}");
}
```

`collect` can also short-circuit: gathering into `Result<Vec<_>, _>`
stops at the first error and returns it, instead of a partial vector.

```rust,editable
fn main() {
    let words = vec!["10", "20", "oops", "30"];

    // Parsing stops at "oops"; the numbers before it are discarded.
    let parsed: Result<Vec<i32>, _> =
        words.iter().map(|w| w.parse()).collect();
    println!("parsed: {:?}", parsed);

    let good = vec!["1", "2", "3"];
    let ok: Result<Vec<i32>, _> =
        good.iter().map(|w| w.parse()).collect();
    println!("ok: {:?}", ok);
}
```

How you borrow the collection matters:

| Form | What it yields | Collection afterwards |
|---|---|---|
| `iter()` | `&T` | Borrowed, still usable |
| `iter_mut()` | `&mut T` | Mutably borrowed, still usable |
| `into_iter()` | `T` | Moved (or borrowed for `&Vec`), often consumed |

```rust,editable
fn main() {
    let mut values = vec![1, 2, 3];

    for v in values.iter_mut() {
        *v *= 10;
    }
    // `iter_mut` only borrowed, so the vector is still ours.
    println!("scaled: {:?}", values);

    let owned_sum: i32 = values.into_iter().sum();
    // `values` was moved and can no longer be used here.
    println!("sum: {owned_sum}");
}
```

### See also:

[`std::iter::Iterator`][iter], [`collect`][collect], and
[`Iterator::fold`][fold].

[iter]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
[collect]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect
[fold]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold

### Exercise: Fix a consumed iterator

Task: Fix the program so the filtered list prints and the count is correct.

<details><summary>Hint</summary>

One consumer exhausts the iterator, so a second consumer needs its own iterator over the same data.

</details>

<details><summary>Solution</summary>

```rust,editable
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];

    // `is_even` is a closure, so each call builds a fresh iterator.
    let is_even = || numbers.iter().filter(|n| *n % 2 == 0);

    let evens: Vec<_> = is_even().copied().collect();
    let count = is_even().count();

    println!("evens: {:?}, count: {}", evens, count);
}
```
</details>

# HashSet

Consider a `HashSet` as a `HashMap` where we just care about the keys (
`HashSet<T>` is, in actuality, just a wrapper around `HashMap<T, ()>`).

"What's the point of that?" you ask. "I could just store the keys in a `Vec`."

A `HashSet`'s unique feature is that it is guaranteed to not have duplicate
elements. That's the contract that any set collection fulfills. `HashSet` is
just one implementation. (see also: [`BTreeSet`][treeset])

If you insert a value that is already present in the `HashSet`, (i.e. the new
value is equal to the existing and they both have the same hash), then the new
value will replace the old.

This is great for when you never want more than one of something, or when you
want to know if you've already got something.

But sets can do more than that.

Sets have 4 primary operations (all of the following calls return an iterator):

- `union`: get all the unique elements in both sets.

- `difference`: get all the elements that are in the first set but not the
  second.

- `intersection`: get all the elements that are only in *both* sets.

- `symmetric_difference`: get all the elements that are in one set or the other,
  but *not* both.

Try all of these in the following example:

```rust,editable,ignore,mdbook-runnable
use std::collections::HashSet;

fn main() {
    let mut a: HashSet<i32> = vec![1i32, 2, 3].into_iter().collect();
    let mut b: HashSet<i32> = vec![2i32, 3, 4].into_iter().collect();

    assert!(a.insert(4));
    assert!(a.contains(&4));

    // `HashSet::insert()` returns false if
    // there was a value already present.
    assert!(b.insert(4), "Value 4 is already in set B!");
    // FIXME ^ Comment out this line

    b.insert(5);

    // If a collection's element type implements `Debug`,
    // then the collection implements `Debug`.
    // It usually prints its elements in the format `[elem1, elem2, ...]`
    println!("A: {:?}", a);
    println!("B: {:?}", b);

    // Print [1, 2, 3, 4, 5] in arbitrary order
    println!("Union: {:?}", a.union(&b).collect::<Vec<&i32>>());

    // This should print [1]
    println!("Difference: {:?}", a.difference(&b).collect::<Vec<&i32>>());

    // Print [2, 3, 4] in arbitrary order.
    println!(
        "Intersection: {:?}",
        a.intersection(&b).collect::<Vec<&i32>>()
    );

    // Print [1, 5]
    println!(
        "Symmetric Difference: {:?}",
        a.symmetric_difference(&b).collect::<Vec<&i32>>()
    );
}
```

(Examples are adapted from the [documentation.][hash-set])

[treeset]: https://doc.rust-lang.org/std/collections/struct.BTreeSet.html
[hash-set]: https://doc.rust-lang.org/std/collections/struct.HashSet.html#method.difference

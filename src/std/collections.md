# More collections

Beyond `Vec`, `String`, and `HashMap`, the standard library ships
ordered and double-ended containers that cover most everyday needs.

`BTreeMap` and `BTreeSet` keep keys sorted, so iteration order is
deterministic. `VecDeque` pushes and pops cheaply from both ends, and
`BinaryHeap` is a max-heap that yields the largest element first.

```rust,editable
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque};

fn main() {
    // Sorted by key: apple, cherry, pear.
    let mut scores = BTreeMap::new();
    scores.insert("pear", 3);
    scores.insert("apple", 5);
    scores.insert("cherry", 4);
    for (fruit, score) in &scores {
        println!("{fruit}: {score}");
    }

    let mut seen = BTreeSet::new();
    seen.insert(3);
    seen.insert(1);
    seen.insert(2);
    println!("sorted set: {:?}", seen);

    let mut queue = VecDeque::new();
    queue.push_back("middle");
    queue.push_front("first");
    queue.push_back("last");
    println!("front: {:?}", queue.pop_front());

    let mut heap = BinaryHeap::new();
    heap.push(1);
    heap.push(5);
    heap.push(2);
    println!("largest: {:?}", heap.pop());
}
```

For counting and caches, `HashMap::entry` inserts a default only when
the key is missing, then hands back a mutable reference in one lookup:

```rust,editable
use std::collections::HashMap;

fn main() {
    let mut pages = HashMap::new();
    // Insert the default `0` for a new key, then add one visit.
    *pages.entry("index").or_insert(0) += 1;
    *pages.entry("index").or_insert(0) += 1;
    *pages.entry("about").or_insert(0) += 1;

    println!("visits: {:?}", pages);
}
```

### See also:

[`std::collections`][collections], [`HashMap::entry`][entry],
[`BTreeMap`][btreemap], [`VecDeque`][vecdeque], and [`BinaryHeap`][heap].

[collections]: https://doc.rust-lang.org/std/collections/index.html
[entry]: https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html
[btreemap]: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
[vecdeque]: https://doc.rust-lang.org/std/collections/struct.VecDeque.html
[heap]: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html

### Exercise: Count words with entry

Task: Count how often each word appears in a sentence using the `entry` API.

<details><summary>Hint</summary>

Look up each word once per occurrence and bump a default of zero held behind the entry.

</details>

<details><summary>Solution</summary>

```rust,editable
use std::collections::HashMap;

fn main() {
    let sentence = "the quick brown fox jumps over the lazy fox";
    let mut counts: HashMap<&str, usize> = HashMap::new();

    for word in sentence.split_whitespace() {
        *counts.entry(word).or_insert(0) += 1;
    }

    assert_eq!(counts["the"], 2);
    assert_eq!(counts["fox"], 2);
    assert_eq!(counts["quick"], 1);
    println!("counts: {:?}", counts);
}
```
</details>

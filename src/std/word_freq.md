# Capstone: word frequency

This page combines [More collections](collections.md) with [Iterators
in depth](iter.md): count words with `HashMap::entry`, then rank them.
A `BTreeMap` keeps the alphabetical listing sorted for free, and a
`sort_by` on the collected pairs produces the leaderboard.

```rust,editable
use std::collections::BTreeMap;

fn word_freq(text: &str) -> BTreeMap<&str, usize> {
    let mut freq = BTreeMap::new();
    for word in text.split_whitespace() {
        *freq.entry(word).or_insert(0) += 1;
    }
    freq
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the fox";
    let freq = word_freq(text);

    // `BTreeMap` iterates in key order, so the listing is alphabetical.
    for (word, count) in &freq {
        println!("{word}: {count}");
    }

    assert_eq!(freq["the"], 3);
    assert_eq!(freq["fox"], 2);
    assert_eq!(freq["dog"], 1);
}
```

Counting and ranking stay separate: the map owns the facts, and each
report (`for` loop, top-N, histogram) is a different consumer of the
same data.

### See also:

[More collections](collections.md) and [Iterators in
depth](iter.md).

### Exercise: Report the top three words

Task: Print the three most frequent words in descending order of count.

<details><summary>Hint</summary>

Collect the pairs into a vector first, since maps cannot reorder themselves by value.

</details>

<details><summary>Solution</summary>

```rust,editable
use std::collections::BTreeMap;

fn word_freq(text: &str) -> BTreeMap<&str, usize> {
    let mut freq = BTreeMap::new();
    for word in text.split_whitespace() {
        *freq.entry(word).or_insert(0) += 1;
    }
    freq
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the fox";
    let freq = word_freq(text);

    let mut pairs: Vec<(&str, usize)> =
        freq.iter().map(|(&w, &c)| (w, c)).collect();
    pairs.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));

    let top3: Vec<(&str, usize)> = pairs.into_iter().take(3).collect();
    assert_eq!(top3[0], ("the", 3));
    assert_eq!(top3[1], ("fox", 2));
    println!("top three: {:?}", top3);
}
```
</details>

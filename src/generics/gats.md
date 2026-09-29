# Generic Associated Types

An associated type can itself be generic: `type Item<'a>` declares a
family of types, one per lifetime. Stable since 1.65, Generic Associated
Types (GATs) express *lending* patterns — iterators that hand out
references tied to the iterator itself, which plain associated types
cannot name.

```rust,editable
// Each call to `next` lends out data for as long as the borrow lasts.
trait LendingIterator {
    type Item<'a> where Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>>;
}

// Yields sliding windows into a slice.
struct Windows<'s> {
    slice: &'s [u32],
    size: usize,
    pos: usize,
}

impl<'s> LendingIterator for Windows<'s> {
    type Item<'a> where Self: 'a = &'a [u32];

    fn next(&mut self) -> Option<Self::Item<'_>> {
        let window = self.slice.get(self.pos..self.pos + self.size)?;
        self.pos += 1;
        Some(window)
    }
}
fn main() {
    let data = vec![1, 2, 3, 4];
    let mut windows = Windows { slice: &data, size: 2, pos: 0 };
    while let Some(w) = windows.next() {
        println!("{:?}", w);
    }
}
```

### See also:

[Associated types][assoc], [Lifetimes][lifetimes], and [Iterators][iter].

[assoc]: ../generics/assoc_items/types.md
[lifetimes]: ../scope/lifetime.md
[iter]: ../trait/iter.md

### Exercise: Lend lines of a string

Task: Implement `LendingIterator` for a type that yields one line at a time.

<details><summary>Hint</summary>

One iterator method on string slices already splits off the next line and reports what remains.

</details>

<details><summary>Solution</summary>

```rust,editable
trait LendingIterator {
    type Item<'a> where Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>>;
}

struct Lines<'s> {
    rest: &'s str,
}

impl<'s> LendingIterator for Lines<'s> {
    type Item<'a> where Self: 'a = &'a str;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        if self.rest.is_empty() {
            return None;
        }
        let (line, rest) = match self.rest.split_once('\n') {
            Some((line, rest)) => (line, rest),
            None => (self.rest, ""),
        };
        self.rest = rest;
        Some(line)
    }
}

fn main() {
    let text = "one\ntwo\nthree";
    let mut lines = Lines { rest: text };
    while let Some(line) = lines.next() {
        println!("line: {}", line);
    }
}
```
</details>

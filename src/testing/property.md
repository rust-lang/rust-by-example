# Property testing

Unit tests check examples; property tests check *invariants* across
hundreds of generated inputs. The [`proptest`][proptest] crate generates
random cases (and shrinks failures to minimal reproducers) for any type
implementing `Arbitrary` — integers, strings, vectors, and your own
types with a derived implementation.

```rust,ignore
// Cargo.toml: proptest = "1"

use proptest::prelude::*;

// Reversing twice must return the original vector, for *every* vector.
proptest! {
    #[test]
    fn reverse_twice_is_identity(v: Vec<u32>) {
        let mut twice = v.clone();
        twice.reverse();
        twice.reverse();
        prop_assert_eq!(twice, v);
    }
}
```

```shell
$ cargo test reverse_twice_is_identity
# On failure, proptest prints the shrunk minimal input and saves a
# seed in proptest-regressions/ for a deterministic replay.
```

Reach for properties when examples feel arbitrary: round-trips
(serialize then parse), idempotence (formatting twice), model
agreement (your code vs. a naive version), and preservation (sort keeps
the length). One property often replaces a table of hand-picked cases.

### See also:

[Unit testing](unit_testing.md) and [`proptest`][proptest].

[proptest]: https://docs.rs/proptest/latest/proptest/

### Exercise: Test the list's string form

Task: Write a property asserting that prepending an element grows the list's string form accordingly.

<details><summary>Hint</summary>

The existing length method gives an independent oracle for how the representation should change.

</details>

<details><summary>Solution</summary>

```rust,ignore
// Cargo.toml: proptest = "1"

use proptest::prelude::*;

use crate::List::*;

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

proptest! {
    #[test]
    fn prepend_grows_stringify(elems: Vec<u32>, extra: u32) {
        let mut list = List::new();
        for e in &elems {
            list = list.prepend(*e);
        }
        let before = list.len() as usize;
        list = list.prepend(extra);
        // One more element means one more ", "-separated item before Nil.
        prop_assert_eq!(list.len() as usize, before + 1);
        prop_assert!(list.stringify().starts_with(&extra.to_string()));
    }
}
```
</details>

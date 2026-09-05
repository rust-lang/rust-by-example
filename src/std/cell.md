# `Cell` and `RefCell`

Most borrow rules are checked at compile time, but `Cell` and `RefCell`
move the check for shared references to run time. This allows mutation
through a shared (`&`) reference, which is useful for types that need
interior mutability, such as when a method taking `&self` must update a
cache or counter.

`Cell<T>` stores a value of a type that implements `Copy`. It has no
borrowing: `get` returns a copy and `set` replaces the value.

```rust,editable
use std::cell::Cell;

fn main() {
    let counter = Cell::new(0);

    // `&counter` is shared, yet the value can still change.
    let shared: &Cell<i32> = &counter;
    shared.set(shared.get() + 1);
    shared.set(shared.get() + 1);

    println!("counter: {}", counter.get());
}
```

`RefCell<T>` works for any type, but enforces the borrow rules at run
time instead: either many shared borrows (`borrow`) or one exclusive
borrow (`borrow_mut`), never both at once. Breaking the rule panics
instead of failing to compile, so keep `borrow_mut` scopes short.

```rust,editable
use std::cell::RefCell;

fn main() {
    let log = RefCell::new(Vec::new());

    log.borrow_mut().push("first");
    log.borrow_mut().push("second");

    // Many shared borrows at once are fine.
    let first = log.borrow();
    println!("entries: {}, first: {}", first.len(), first[0]);
}
```

`Ref::map` derives a borrow of part of the contents without copying,
keeping the original borrow alive:

```rust,editable
use std::cell::{Ref, RefCell};

fn main() {
    let pair = RefCell::new(("ferris".to_string(), 42));
    let borrowed = pair.borrow();
    let name: Ref<String> = Ref::map(borrowed, |(name, _)| name);
    println!("name: {}", name);
}
```

### See also:

[`std::cell`][cell], [`Cell`][cell_struct], and [`RefCell`][refcell].

[cell]: https://doc.rust-lang.org/std/cell/index.html
[cell_struct]: https://doc.rust-lang.org/std/cell/struct.Cell.html
[refcell]: https://doc.rust-lang.org/std/cell/struct.RefCell.html

### Exercise: Fix a double borrow panic

Task: Fix the program so both pushes succeed without panicking.

<details><summary>Hint</summary>

Two exclusive borrows of the same value cannot overlap; end each one before starting the next.

</details>

<details><summary>Solution</summary>

```rust,editable
use std::cell::RefCell;

fn main() {
    let log = RefCell::new(Vec::new());

    // Each `borrow_mut` temporary drops at the end of its statement,
    // so the two exclusive borrows never overlap.
    log.borrow_mut().push("first");
    log.borrow_mut().push("second");

    println!("entries: {:?}", log.borrow());
}
```
</details>

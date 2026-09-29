# `MaybeUninit`

Reading uninitialized memory is undefined behavior, even if you never
look at the value — the compiler may assume it cannot happen. When
initialization is genuinely two-phase (FFI out-pointers, buffering,
hand-rolled split borrows), `MaybeUninit<T>` is the sanctioned
scratch space: write first, `assume_init` only after every byte is
initialized.

```rust,editable
use std::mem::MaybeUninit;

fn main() {
    let mut slot = MaybeUninit::<String>::uninit();

    slot.write("ferris".to_string());

    // Safe: `write` fully initialized the slot, so assuming it is
    // initialized upholds the contract. Dropping `slot` itself without
    // `assume_init` would leak; reading before `write` would be UB.
    let value = unsafe { slot.assume_init() };
    println!("{}", value);
}
```

The canonical safe abstraction over raw parts is `split_at_mut`: two
mutable slices from one, proven disjoint by construction.

```rust,editable
fn main() {
    let mut data = [1, 2, 3, 4];
    let len = data.len();
    let ptr = data.as_mut_ptr();

    // Safe: both halves lie inside `data` (which outlives them and has
    // no other live references), and the ranges `[0, len/2)` and
    // `[len/2, len)` do not overlap.
    let (left, right) = unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, len / 2),
            std::slice::from_raw_parts_mut(ptr.add(len / 2), len - len / 2),
        )
    };

    left[0] = 10;
    println!("{:?} {:?}", left, right);
}
```

For foreign functions, combine `repr(C)` layouts with raw pointers at
the boundary, and wrap the result in a safe function immediately — see
[Foreign Function Interface][ffi] for the calling convention side.

### See also:

[`std::mem::MaybeUninit`][maybe], [`std::ptr`][ptr], and [Foreign
Function Interface][ffi].

[maybe]: https://doc.rust-lang.org/std/mem/union.MaybeUninit.html
[ptr]: https://doc.rust-lang.org/std/ptr/index.html
[ffi]: ../std_misc/ffi.md

### Exercise: Write a safe split wrapper

Task: Write a `split_at_mut`-style safe wrapper and state the two invariants that make it sound.

<details><summary>Hint</summary>

The caller upholds one bound on the split point, and the two derived slices must never overlap.

</details>

<details><summary>Solution</summary>

```rust,editable
fn split_at_mut<T>(slice: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    // Invariant 1: the split point is in bounds, so both ranges are
    // inside the allocation. Invariant 2: the ranges `[0, mid)` and
    // `[mid, len)` are disjoint, so no two `&mut` alias.
    assert!(mid <= slice.len());
    let ptr = slice.as_mut_ptr();
    let len = slice.len();
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main() {
    let mut data = [1, 2, 3, 4, 5];
    let (left, right) = split_at_mut(&mut data, 2);
    left[0] = 10;
    right[0] = 30;
    println!("{:?} | {:?}", left, right);
}
```
</details>

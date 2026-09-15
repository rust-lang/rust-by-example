# Ownership and moves

Because variables are in charge of freeing their own resources,
**resources can only have one owner**. This prevents resources
from being freed more than once. Note that not all variables own
resources (e.g. [references]).

When doing assignments (`let x = y`) or passing function arguments by value
(`foo(x)`), the *ownership* of the resources is transferred. In Rust-speak,
this is known as a *move*.

After moving resources, the previous owner can no longer be used. This avoids
creating dangling pointers.

```rust,editable
// This function takes ownership of the heap allocated memory
fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);

    // `c` is destroyed and the memory freed
}

fn main() {
    // _Stack_ allocated integer
    let x = 5u32;

    // *Copy* `x` into `y` - no resources are moved
    let y = x;

    // Both values can be independently used
    println!("x is {}, and y is {}", x, y);

    // `a` is a pointer to a _heap_ allocated integer
    let a = Box::new(5i32);

    println!("a contains: {}", a);

    // *Move* `a` into `b`
    let b = a;
    // The pointer address of `a` is copied (not the data) into `b`.
    // Both are now pointers to the same heap allocated data, but
    // `b` now owns it.

    // Error! `a` can no longer access the data, because it no longer owns the
    // heap memory
    //println!("a contains: {}", a);
    // TODO ^ Try uncommenting this line

    // This function takes ownership of the heap allocated memory from `b`
    destroy_box(b);

    // Since the heap memory has been freed at this point, this action would
    // result in dereferencing freed memory, but it's forbidden by the compiler
    // Error! Same reason as the previous Error
    //println!("b contains: {}", b);
    // TODO ^ Try uncommenting this line
}
```

[references]: ../flow_control/match/destructuring/destructure_pointers.md

### Exercise: Lend instead of moving

Task: Change `destroy_box` to borrow its argument so `b` stays usable afterwards.

<details><summary>Hint</summary>

Receiving a reference instead of ownership leaves the owner untouched, and the call site lends with one character.

</details>

<details><summary>Solution</summary>

```rust,editable
// This function borrows the heap allocated memory instead of taking it.
fn inspect_box(c: &Box<i32>) {
    println!("Inspecting a box that contains {}", c);
}

fn main() {
    let a = Box::new(5i32);

    // *Move* `a` into `b`; `a` can no longer be used.
    let b = a;

    // Lend `b` instead of moving it, so it stays usable below.
    inspect_box(&b);
    println!("b still contains: {}", b);
}
```

</details>

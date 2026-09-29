# Testcase: empty bounds

A consequence of how bounds work is that even if a `trait` doesn't
include any functionality, you can still use it as a bound. `Eq` and
`Copy` are examples of such `trait`s from the `std` library.

```rust,editable
struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

// These functions are only valid for types which implement these
// traits. The fact that the traits are empty is irrelevant.
fn red<T: Red>(_: &T)   -> &'static str { "red" }
fn blue<T: Blue>(_: &T) -> &'static str { "blue" }

fn main() {
    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey   = Turkey;

    // `red()` won't work on a blue jay nor vice versa
    // because of the bounds.
    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
    //println!("A turkey is {}", red(&_turkey));
    // ^ TODO: Try uncommenting this line.
}
```

### Exercise: Add a third empty bound

Task: Add a new empty trait, implement it for one type, and bound a function on it.

<details><summary>Hint</summary>

The new trait follows the same shape as the existing two, and only the implementing type satisfies the new bound.

</details>

<details><summary>Solution</summary>

```rust,editable
struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}
trait Green {}

impl Red for Cardinal {}
impl Blue for BlueJay {}
impl Green for Turkey {}

fn red<T: Red>(_: &T) -> &'static str { "red" }
fn blue<T: Blue>(_: &T) -> &'static str { "blue" }
fn green<T: Green>(_: &T) -> &'static str { "green" }

fn main() {
    let turkey = Turkey;
    assert_eq!(green(&turkey), "green");
    println!("A turkey is {}", green(&turkey));
}
```

</details>

### See also:

[`std::cmp::Eq`][eq], [`std::marker::Copy`][copy], and [`trait`s][traits]

[eq]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
[copy]: https://doc.rust-lang.org/std/marker/trait.Copy.html
[traits]: ../../trait.md

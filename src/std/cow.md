# `Cow`

`Cow<'_, T>` (clone-on-write) holds either borrowed data or an owned
value of the same shape. Functions taking `Cow<str>` accept both `&str`
(without allocating) and `String` (without conversion), and only clone
when mutation is actually needed.

```rust,editable
use std::borrow::Cow;

fn main() {
    // Borrowed: no allocation, just a reference wrapper.
    let borrowed: Cow<str> = Cow::Borrowed("hello");
    // Owned: takes over an existing allocation.
    let owned: Cow<str> = Cow::Owned("hello".to_string());

    println!("borrowed: {}, owned: {}", borrowed, owned);

    match borrowed {
        Cow::Borrowed(s) => println!("still borrowed: {}", s),
        Cow::Owned(_) => println!("unexpectedly owned"),
    }
}
```

`to_mut` clones the data on first write if it was borrowed, then hands
out a mutable reference. `into_owned` consumes the `Cow` and returns an
owned value, cloning only in the borrowed case.

```rust,editable
use std::borrow::Cow;

fn shout(mut text: Cow<str>) -> String {
    // Borrows clone here; already-owned values mutate in place.
    text.to_mut().make_ascii_uppercase();
    text.into_owned()
}

fn main() {
    println!("{}", shout(Cow::Borrowed("hello")));
    println!("{}", shout(Cow::Owned("world".to_string())));
}
```

### See also:

[`std::borrow::Cow`][cow] and [`ToOwned`][toowned].

[cow]: https://doc.rust-lang.org/std/borrow/enum.Cow.html
[toowned]: https://doc.rust-lang.org/std/borrow/trait.ToOwned.html

### Exercise: Describe borrowed or owned input

Task: Write a function that reports whether a `Cow<str>` is borrowed or owned.

<details><summary>Hint</summary>

Matching on the two variants tells you which case you hold without moving the contents.

</details>

<details><summary>Solution</summary>

```rust,editable
use std::borrow::Cow;

fn describe(c: &Cow<str>) -> &'static str {
    match c {
        Cow::Borrowed(_) => "borrowed",
        Cow::Owned(_) => "owned",
    }
}

fn main() {
    let a: Cow<str> = Cow::Borrowed("hi");
    let b: Cow<str> = Cow::Owned("hi".to_string());
    println!("a is {}, b is {}", describe(&a), describe(&b));
}
```
</details>

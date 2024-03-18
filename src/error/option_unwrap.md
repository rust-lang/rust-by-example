# `Option` & `unwrap`

In the last example, we showed that we can induce program failure at will. We
told our program to `panic` if we drink a sugary lemonade. But what if we expect
*some* drink but don't receive one? That case would be just as bad, so it needs
to be handled!

We *could* test this against the null string (`""`) as we do with a lemonade.
Since we're using Rust, let's instead have the compiler point out cases where
there's no drink.

An `enum` called `Option<T>` in the `std` library is used when absence is a
possibility. It manifests itself as one of two "options":

- `Some(T)`: An element of type `T` was found
- `None`: No element was found

These cases can either be explicitly handled via `match` or implicitly with
`unwrap`. Implicit handling will either return the inner element or `panic`.

Note that it's possible to manually customize `panic` with [expect][expect], but
`unwrap` otherwise leaves us with a less meaningful output than explicit
handling. In the following example, explicit handling yields a more controlled
result while retaining the option to `panic` if desired.

```rust,editable,ignore,mdbook-runnable
// The adult has seen it all, and can handle any drink well.
// All drinks are handled explicitly using `match`.
fn give_adult(drink: Option<&str>) {
    // Specify a course of action for each case.
    match drink {
        Some("lemonade") => println!("Yuck! Too sugary."),
        Some(inner) => println!("{}? How nice.", inner),
        None => println!("No drink? Oh well."),
    }
}

// Others will `panic` before drinking sugary drinks.
// All drinks are handled implicitly using `unwrap`.
fn drink(drink: Option<&str>) {
    // `unwrap` returns a `panic` when it receives a `None`.
    let inside = drink.unwrap();
    if inside == "lemonade" {
        panic!("AAAaaaaa!!!!");
    }

    println!("I love {}s!!!!!", inside);
}

fn main() {
    let water = Some("water");
    let lemonade = Some("lemonade");
    let void = None;

    give_adult(water);
    give_adult(lemonade);
    give_adult(void);

    let coffee = Some("coffee");
    let nothing = None;

    drink(coffee);
    drink(nothing);
}
```

[expect]: https://doc.rust-lang.org/std/option/enum.Option.html#method.expect

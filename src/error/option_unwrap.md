# `Option` & `unwrap`

In the last example, we showed that we can induce program failure at will. 
We told our program to `panic` if the princess received an inappropriate 
gift - a snake. But what if the princess expected a gift and didn't receive 
one? That case would be just as bad, so it needs to be handled!

We *could* test this against the null string (`""`) as we do with a snake. 
Since we're using Rust, let's instead have the compiler point out cases 
where there's no gift.

An `enum` called `Option<T>` in the `std` library is used when absence is a 
possibility. It manifests itself as one of two "options":

* `Some(T)`: An element of type `T` was found
* `None`: No element was found

These cases can either be explicitly handled via `match` or implicitly with 
`unwrap`. Implicit handling will either return the inner element or `panic`.

Note that it's possible to manually customize `panic` with [expect][expect], 
but `unwrap` otherwise leaves us with a less meaningful output than explicit 
handling. In the following example, explicit handling yields a more 
controlled result while retaining the option to `panic` if desired.

```rust,editable,ignore,mdbook-runnable
// The commoner has seen it all, and can handle any gift well.
// All gifts are handled explicitly using `match`.
fn give_commoner(gift: Option<&str>) {
    // Specify a course of action for each case.
    match gift {
        Some("snake") => println!("Yuck! I'm putting this snake back in the forest."),
        Some(inner)   => println!("{}? How nice.", inner),
        None          => println!("No gift? Oh well."),
    }
}

// Our sheltered princess will `panic` at the sight of snakes.
// All gifts are handled implicitly using `unwrap`.
fn give_princess(gift: Option<&str>) {
    // `unwrap` returns a `panic` when it receives a `None`.
    let inside = gift.unwrap();
    if inside == "snake" { panic!("AAAaaaaa!!!!"); }

    println!("I love {}s!!!!!", inside);
}

fn main() {
    let food  = Some("cabbage");
    let snake = Some("snake");
    let void  = None;

    give_commoner(food);
    give_commoner(snake);
    give_commoner(void);

    let bird = Some("robin");
    let nothing = None;

    give_princess(bird);
    give_princess(nothing);
}
```

[expect]: https://doc.rust-lang.org/std/option/enum.Option.html#method.expect

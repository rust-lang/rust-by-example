For some use cases, `match` is awkward. For example:

```rust
// Make `optional` of type `Option<i32>`
let optional = Some(7);

match optional {
    Some(i) => {
        println!("This is a really long string and `{:?}`", i);
        // ^ Needed 2 indentations just so we could destructure
        // `i` from the option.
    },
    _ => {},
    // ^ Required because `match` is exhaustive. Doesn't it seem
    // like wasted space?
};

```

`if let` is cleaner for this use case and in addition allows various
failure options to be specified:

{if_let.play}

### See also:

[`enum`][enum], [`Option`][option], and the [RFC][if_let_rfc]

[enum]: /custom_types/enum.html
[if_let_rfc]: https://github.com/rust-lang/rfcs/pull/160
[option]: /std/option.html

# Default

The [`Default`][default] trait provides a useful default value for a type. Many
types in the standard library implement it, and it can be derived for your own
types with `#[derive(Default)]`, which gives every field its own default.

```rust,editable
#[derive(Default, Debug)]
struct MyConfig {
    name: String,
    count: u32,
    verbose: bool,
}

fn main() {
    // `Default::default()` builds a value with every field defaulted.
    let config = MyConfig::default();
    println!("{config:?}");

    // The struct update syntax takes the remaining fields from
    // `Default::default()`, so you only set the ones you care about.
    let custom = MyConfig {
        name: "app".to_string(),
        ..Default::default()
    };
    println!("{custom:?}");
}
```

You can also implement `Default` by hand when the default you want is not just
the field-by-field default:

```rust,editable
struct Port(u16);

impl Default for Port {
    fn default() -> Self {
        Port(8080)
    }
}

fn main() {
    let Port(port) = Port::default();
    println!("default port: {port}");
}
```

### When to use it

`Default` is a good fit when a type has an obvious "empty" or "zero" value. It
also works with APIs that rely on it, such as the struct update syntax above or
[`Option::unwrap_or_default`][unwrap-or-default].

Prefer a regular constructor (for example an associated `new` function) when a
type has no meaningful default, or when building a value needs arguments or can
fail. Implementing `Default` in those cases forces an arbitrary value, which can
hide bugs.

### See also

[`derive`][derive] and [`Option`][option].

[default]: https://doc.rust-lang.org/std/default/trait.Default.html
[derive]: derive.md
[option]: https://doc.rust-lang.org/std/option/enum.Option.html
[unwrap-or-default]: https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or_default

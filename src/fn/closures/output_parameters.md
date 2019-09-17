# As output parameters

Closures as input parameters are possible, so returning closures as
output parameters should also be possible. However, anonymous
closure types are, by definition, unknown, so we have to use
`impl Trait` to return them.

The valid traits for returns are slightly different than before:

* `Fn`: normal
* `FnMut`: normal
* `FnOnce`: There are some unusual things at play here, so the [`FnBox`][fnbox]
  type is currently needed, and is unstable. This is expected to change in
  the future.

Beyond this, the `move` keyword must be used, which signals that all captures
occur by value. This is required because any captures by reference would be
dropped as soon as the function exited, leaving invalid references in the
closure.

```rust,editable
fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();

    move || println!("This is a: {}", text)
}

fn main() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();

    fn_plain();
    fn_mut();
}
```

### See also:

[`Fn`][fn], [`FnMut`][fnmut], [Generics][generics] and [impl Trait][impltrait].

[fn]: https://doc.rust-lang.org/std/ops/trait.Fn.html
[fnmut]: https://doc.rust-lang.org/std/ops/trait.FnMut.html
[fnbox]: https://doc.rust-lang.org/std/boxed/trait.FnBox.html
[generics]: ../../generics.md
[impltrait]: ../../trait/impl_trait.md

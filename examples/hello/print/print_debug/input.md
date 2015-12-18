All types which want to use `std::fmt` formatting `traits` require an
implementation to be printable. Automatic implementations are only provided
for types such as in the `std` library. All others *must* be manually
implemented somehow.

The `fmt::Debug` `trait` makes this very straightforward. *All* types can
`derive` (automatically create) the `fmt::Debug` implementation. This is
not true for `fmt::Display` which must be manually implemented.

```rust
// This structure cannot be printed either with `fmt::Display` or
// with `fmt::Debug`
struct UnPrintable(i32);

// The `derive` attribute automatically creates the implementation
// required to make this `struct` printable with `fmt::Debug`.
#[derive(Debug)]
struct DebugPrintable(i32);
```

All `std` library types automatically are printable with `{:?}` too:

{debug.play}

So `fmt::Debug` definitely makes this printable but sacrifices some
elegance. Manually implementing `fmt::Display` will fix that.

### See also

[attributes][attributes], [`derive`][derive], [`std::fmt`][fmt],
and [`struct`][structs]

[attributes]: http://doc.rust-lang.org/reference.html#attributes
[derive]: /trait/derive.html
[fmt]: http://doc.rust-lang.org/std/fmt/
[structs]: /custom_types/structs.html


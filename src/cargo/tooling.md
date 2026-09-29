# Tooling: fmt, clippy, doc, audit

Three commands keep a Rust project healthy, and all of them run in CI.
`cargo fmt` normalizes layout, `cargo clippy` lints for common
mistakes, and `cargo doc` builds API documentation from doc comments.

```shell
$ cargo fmt --all -- --check   # fail CI if anything is unformatted
$ cargo clippy --all-targets --all-features -- -D warnings
$ cargo doc --no-deps --open   # docs for your crates only, then open
```

Fix formatting with bare `cargo fmt`, and read a lint's full
explanation before silencing it — most clippy warnings are genuine
bugs (`needless_range_loop`, `unwrap_used` in libraries) rather than
style. Deny-by-default in CI, allow-by-exception in code with a
commented reason:

```rust,ignore
#[allow(clippy::too_many_arguments)] // builder pattern defeats the lint's purpose here
fn connect(host: &str, port: u16, user: &str, password: &str,
           database: &str, timeout_secs: u64, retries: u32) -> Connection {
    // ...
    # todo!()
}
```

For the supply chain, audit dependencies regularly:

```shell
$ cargo install cargo-audit
$ cargo audit                  # known CVEs in your lockfile
$ cargo update -p some-crate   # bump one dependency deliberately
```

### See also:

[Tests](test.md), [Conventions](conventions.md), and [the Clippy
lint list][clippy].

[clippy]: https://rust-lang.github.io/rust-clippy/master/

### Exercise: Silence one lint properly

Task: Fix a `clippy::needless_range_loop` warning the idiomatic way instead of allowing it.

<details><summary>Hint</summary>

Iterating elements directly removes the indexing the lint complains about.

</details>

<details><summary>Solution</summary>

```rust,editable
fn main() {
    let values = vec![1, 2, 3, 4];

    // Before: `for i in 0..values.len() { total += values[i]; }`
    // lints `needless_range_loop`. Iterate the elements instead:
    let mut total = 0;
    for v in &values {
        total += v;
    }
    println!("total: {}", total);
}
```
</details>

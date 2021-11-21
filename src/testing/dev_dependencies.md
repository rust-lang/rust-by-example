# Development dependencies

Sometimes there is a need to have dependencies for tests (or examples,
or benchmarks) only. Such dependencies are added to `Cargo.toml` in the
`[dev-dependencies]` section. These dependencies are not propagated to other
packages which depend on this package.

One such example is using a crate that extends standard `assert!` macros.  
File `Cargo.toml`:

```toml
# standard crate data is left out
[dev-dependencies]
pretty_assertions = "1"
```

File `src/lib.rs`:

```rust,ignore
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq; // crate for test-only use. Cannot be used in non-test code.

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
```

## See Also
[Cargo][cargo] docs on specifying dependencies.

[cargo]: http://doc.crates.io/specifying-dependencies.html

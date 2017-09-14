# Testing

Functions can be tested by using these [attributes][attributes]:

* `#[test]` marks a function as a unit test. The function must take zero
parameters and return nothing.
* `#[should_panic]` marks a function as a panicking test.

```rust,editable
// Conditionally compile `main` only when the test-suite is *not* being run.
#[cfg(not(test))]
fn main() {
    println!("If you see this, the tests were not compiled nor ran!");
}

// Conditionally compile the module `test` only when the test-suite is run.
#[cfg(test)]
mod test {
    // A helper function `distance_test` will need.
    fn distance(a: (f32, f32), b: (f32, f32)) -> f32 {
        (
            (b.0 - a.0).powi(2) +
            (b.1 - a.1).powi(2)
        ).sqrt()
    }

    #[test]
    fn distance_test() {
        assert!(distance((0f32, 0f32), (1f32, 1f32)) == (2f32).sqrt());
    }
    
    #[test]
    #[should_panic]
    fn failing_test() {
        assert!(1i32 == 2i32);
    }
}
```

Tests can be run with `cargo test` or `rustc --test`.

```bash
$ rustc --test unit_test.rs
$ ./unit_test 

running 2 tests
test test::distance_test ... ok
test test::failing_test ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured
```

If `--test` were not included, then this would happen

```bash
$ rustc unit_test.rs
$ ./unit_test
If you see this, the tests were not compiled nor ran!
```

### See also:

[attributes][attributes], [conditional compilation][cfg], and [`mod`][mod].

[attributes]: /attribute.html
[cfg]: /attribute/cfg.html
[mod]: /mod.html

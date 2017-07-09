Functions can be tested by using these [attributes][attributes]:

* `#[test]` marks a function as a unit test. The function must take zero
parameters and return nothing.
* `#[should_panic]` marks a function as a panicking test.

{unit_test.rs}

Tests can be run with `cargo test` or `rustc --test`.

```
$ rustc --test unit_test.rs
$ ./unit_test

running 2 tests
test test::distance_test ... ok
test test::failing_test ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured
```

If `--test` were not included, then this would happen

```
$ rustc unit_test.rs
$ ./unit_test
If you see this, the tests were not compiled nor ran!
```

### Смотрите также:

[attributes][attributes], [conditional compilation][cfg], and [`mod`][mod].

[attributes]: ../attribute.html
[cfg]: ../attribute/cfg.html
[mod]: ../mod.html

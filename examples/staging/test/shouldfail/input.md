If a test's expected behavior is to fail, mark it with the `#[should_fail]`
attribute. The test will now pass if and only if the task fails.

{should_fail.rs}

```
$ rustc --test should_fail.rs
$ ./should_fail
running 2 tests
test non_failing_test ... FAILED
test out_of_bounds_test ... ok

failures:
    non_failing_test

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured
```

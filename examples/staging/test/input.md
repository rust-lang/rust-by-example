To mark a function as a unit test, place `#[test]` above it. A test function
must take no parameters and does not return a value. Tests in Rust are
considered failed tests if they encounter a task failure.

{unit-test.rs}

To run unit tests, you must first build a test runner. Do this by adding the
`--test` flag to the Rust compiler. Afterwards, running the resulting executable
will print out the results.

```
$ rustc --test unit-test.rs
$ ./unit-test
running 2 tests
test distance_test ... ok
test translate_test ... FAILED

failures:

---- translate_test stdout ----
    task 'translate_test' failed at 'assertion failed: new_x == 6.0', unit-test.rs:30

failures:
    translate_test

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured
```

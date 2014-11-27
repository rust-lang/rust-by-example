An item marked with `#[cfg(test)]` will only be compiled when a test runner is
built. This makes it possible to define helper functions and modules for
testing that don't get included in a normal build.

{condtest.play}

Here is the test results of the previous file:

```
running 1 test
test square_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

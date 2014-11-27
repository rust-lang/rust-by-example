By default, the standard output and standard error streams of a test are
suppressed. This is done because the test are run in parallel, possibly making
custom output difficult to read.

{abs_test.rs}

With no flags, the printed text will not appear:

```
$ rustc --test abs_test.rs
$ ./abs_test
running 1 test
test abs_positive_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

To change this, simply pass `--nocapture` to the test runner:

```
$ ./abs_test --nocapture
running 1 test
abs_positive_test ran with n = -20
test abs_positive_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

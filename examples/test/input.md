To mark a function as a unit test, place `#[test]` (equivalent
to `#[cfg(test)]`) above any function.  The function must take
no parameters and return nothing.

{unit-test.rs}

If you want the test to fail, just put `#[should_fail]` under `#[test]`.

{fail.rs}

To run unit tests, add either the `--test` or `--cfg test` flag to the
command.  When using either flag, you do not need a `main()` function
as the executable will only include and run the functions flagged with
`#[test]` or `#[cfg(test)]`.  If you don't pass the `--test` flag, `rustc` will
ignore any functions flagged with `#[test]`.  That means calling any function
flagged with `#[test]` will result in `rustc: unresolved name` if it is not
compiled with `--test`.

**note**: programs compiled with `--test` *ignore* information passed to stdout

```
$ rustc --test unit-test.rs
$ ./unit-test
running 1 test
test distance_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

If `--test` were not included, then this would happen

```
$ rustc unit-test.rs
$ ./unit-test
If you see this, the tests were not compiled nor ran!
```
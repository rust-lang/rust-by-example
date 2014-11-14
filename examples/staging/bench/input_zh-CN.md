Rust provides infrastructure for benchmarking via the `Bencher` struct and
the `#[bench]` attribute. Details in the source code below.

{bench.rs}

The source needs to be compiled using the `--test` flag, and the `--bench` flag
must be passed to the resulting binary.

``` bash
$ rustc --test -O bench.rs
$ ./bench --bench
running 2 tests
test iterative_fibonacci ... bench:       191 ns/iter (+/- 16)
test recursive_fibonacci ... bench:     49670 ns/iter (+/- 522)

test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured
```

Rust provides experimental support for SIMD vectors. These SIMD vectors are
exposed as structs (`f32x4`, `u8x16`, etc.), that implement basic operations
(`+`, `-`, `*`, etc) using SIMD instructions under the hood.

{simd.rs}

{simd.out}

Here's a more complex example that sums two `Vec<f32>`, using the `f32x4` type
to operate on 4-element chunks at a time.

{simd_add.rs}

And here's the result of the benchmark:

```
$ rustc -O --test simd_add.rs && ./simd_add --bench
running 4 tests
test test::simd ... ignored
test test::vanilla ... ignored
test bench::simd    ... bench:      1852 ns/iter (+/- 17)
test bench::vanilla ... bench:      8346 ns/iter (+/- 103)

test result: ok. 0 passed; 0 failed; 2 ignored; 2 measured
```

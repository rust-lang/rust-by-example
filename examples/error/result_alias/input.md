What if the specific `Result` type is reused many many times? Then quickly it becomes tedious
to write out the full type name. Instead, a generic alias for the specific `Result` may be
defined.

{alias.play}

This is particularly helpful at a module level because all errors found in a specific module
may have the same `Err` type; a single alias succinctly defines *all* module `Results`. This
is so useful that the std library even supplies one: `io::Result` which refers to IO errors.

### See also:

[`Result`][result] and [`io::Result`][io_result]

[result]: http://doc.rust-lang.org/std/result/enum.Result.html
[io_result]: http://doc.rust-lang.org/std/io/type.Result.html

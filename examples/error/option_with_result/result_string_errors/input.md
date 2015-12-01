Eliminating `unwrap` from the previous example requires more care. The two types in play
being `Option` and `Result`, one valid approach would be to convert both into a `Result`
with a common `Err` type. We will try it with `Err(String)` which seems like a nice first
approximation:

{result_string.play}

This is not too bad but it is hardly as nice as the original (it can still be nicer but
we are not there yet). The question is, does this approach scale well. Consider the next
example.

### See also:

[`Result`][result] and [`io::Result`][io_result]

[result]: http://doc.rust-lang.org/std/result/enum.Result.html
[io_result]: http://doc.rust-lang.org/std/io/type.Result.html

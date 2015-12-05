What if multiple `Results` needed to interact together? Is it still reasonably convenient?
It turns out, not really.

{result_try.play}

What is happening is this approach tries to work with the data without ever removing the `Ok`
wrapper on it. Sometimes it is a good approach but in this case it is really awkward. What if
we could `unwrap` it without possibly inducing `panic`? That is where we are headed next.

### See also:

[`Result`][result] and [`io::Result`][io_result]

[result]: http://doc.rust-lang.org/std/result/enum.Result.html
[io_result]: http://doc.rust-lang.org/std/io/type.Result.html

The previous examples have always been very convenient; a `Result` interacts with the same
`Results` and an `Option` with the same `Option`. Sometimes it is not this easy though;
`Options` and `Results` may have to interact or even `Result<T, Error1>` with
`Result<T, Error2>`.

Here is an example where one returns an `Option` and the other returns an `Result`. Aside
from messy errors provided by `unwrap`, this looks reasonable:

{option_result.play}

### See also:

[`Result`][result] and [`io::Result`][io_result]

[result]: http://doc.rust-lang.org/std/result/enum.Result.html
[io_result]: http://doc.rust-lang.org/std/io/type.Result.html

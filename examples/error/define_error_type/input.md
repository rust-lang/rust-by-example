Rust allows us to define our own error types. In general, a "good" error type:

* Represents different errors with the same type
* Presents nice error messages to the user
* Is easy to compare with other types
    - Good: `Err(EmptyVec)`
    - Bad: `Err("Please use a vector with at least one element".to_owned())`
* Can hold information about the error
    - Good: `Err(BadChar(c, position))`
    - Bad: `Err("+ cannot be used here".to_owned())`

Note that a `String` (which we've been using up to this point) fulfills the
first two criteria, but not the last two. This makes `String` errors verbose
to create and difficult to react to. It should not be necessary to pollute
logic heavy code with `String` formatting simply to display nicely.

{define_error_type.play}

### Смотрите также:

[`Result`][result] and [`io::Result`][io_result]

[result]: http://doc.rust-lang.org/std/result/enum.Result.html
[io_result]: http://doc.rust-lang.org/std/io/type.Result.html
[inplace]: ../error/option_with_result/result_string_errors.html

We have been using `Strings` as errors for a while. In fact, this is somewhat limiting as
an error type. Below are the criteria for a good error type. `String` nicely fulfills the first
two but not the second two:

* Represents different errors with the same type
* Presents nice error messages to the user
* Is easily type comparable. Consider comparing these two types:
    - `Err("Please use a vector with at least one element".to_owned())`
    - `Err(EmptyVec)`
* Can hold information about the error. Compare:
    - `Err("+ cannot be used here".to_owned())`
    - `Err(BadChar(c, position))`

This makes `String` errors both difficult to react to and verbose to create. In fact, a nice
looking error message has nothing to do with how the type is structured. It is simply a
consequence of `Display` being implemented for the type. It should not be necessary to
pollute logic heavy code with `String` formatting simply for nice error messages.

{rethink.play}

### See also:

[`Result`][result] and [`io::Result`][io_result]

[result]: http://doc.rust-lang.org/std/result/enum.Result.html
[io_result]: http://doc.rust-lang.org/std/io/type.Result.html
[inplace]: /error/option_with_result/result_string_errors.html

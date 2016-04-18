Previously, we have used the type `Option` to annotate that absence is a possibility. This
absence sometimes appears as an error, for example when `None` is unwrapped. In the more
general case where there may be multiple failure points for a multitude of different reasons,
an `Option` can be replaced by the more general `Result` type. A `Result<T, E>` has these
variants:

* `Ok<T>`: An element `T` was found
* `Err<E>`: An error was found with element `E`

Similar to `Option`, `Result` also contains the `unwrap()` method which yields the element
`T` or calls `panic!()`. So far, this should seem similar to `Option`:

{result.play}

Clearly, panicking on an `Err` leaves an unhelpful error message. Do we even know anything
about libcore that the error is telling us all about? There must be a better way.


### See also:

[`Result`][result]

[result]: http://doc.rust-lang.org/std/result/enum.Result.html

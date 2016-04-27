Previously, we have used the `Option` type to annotate that absence is a possibility. This
absence sometimes appears as an error, such as when `None` is unwrapped. 
When multiple failure points may exist, an `Option` can be replaced by the 
more general `Result` type. A `Result<T, E>` has these variants:

* `Ok<T>`: An element `T` was found
* `Err<E>`: An error was found with element `E`

Similar to `Option`, `Result` also contains the `unwrap()` method which yields the element
`T` or calls `panic!()`. So far, this should seem similar to `Option`:

{result.play}

Clearly, panicking on an `Err` leaves an unhelpful error message. Luckily for us, 
the upcoming combinators are available to help us with errors.


### See also:

[`Result`][result]

[result]: http://doc.rust-lang.org/std/result/enum.Result.html

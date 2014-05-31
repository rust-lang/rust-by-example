We've seen that the `Option` enum can be used as a return value from functions
that may fail, where `None` can be returned to indicate failure. However,
sometimes is important to express *why* an operation failed. To do this we have
the `Result` enum.

The `Result` enum can take on two variants: `Ok` or `Err`. Each variant is a
tuple struct that wraps some value. The `Ok(value)` variant indicates that the
operation succeeded, and wraps the `value` returned by the operation; on the
other hand `Err(why)` indicates that the operation failed, and wraps `why`
which (hopefully) explains the cause of the failure.

`Result` is a generic enum with type signature `Result<T, U>`, and its variants
are also generic: `Ok(T)` and `Err(U)`. Hence, the `Result` enum has extensive
use in the Rust standard library, especially when dealing with I/O.

Let's use `Result` to catch undefined mathematical operations.

{checked.rs}

{result.rs}

{result.out}

Chaining results using match can get pretty untidy; luckily, the `try!` macro
can be used to make things pretty again. The `try!` macro expands to a match
expression, where the `Err(err)` branch expands to an early `return err`, and
the `Ok(ok)` branch expands to the `ok` expression.

{try.rs}

{try.out}

Be sure to check the [documentation](http://static.rust-lang.org/doc/master/std/result/type.Result.html),
as there are many methods to map/compose `Result`.

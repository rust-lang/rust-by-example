By implementing `Display` and `From` for our error type, we enabled
almost all of the `std` library error handling tools. However, we missed
something: the ability to easily `Box` our error type.

The `std` library automatically converts any type that implements the
`Error` trait into the trait object `Box<Error>`, via `From`. To a
library user, this conveniently allows the following:

```rust
fn foo(...) -> Result<T, Box<Error>> { ... }
```

A user may use any variety of external libraries which each provide their own error
types. In order to define a valid `Result<T, E>` type, the user has a few choices:

* define a new wrapper error type around the library's error types
* convert the error types to `String` or another intermediate choice
* `Box` the error types into `Box<Error>` via type erasure

"Boxing" the error type is a common choice. The drawback is that the
underlying error type is only known at runtime and not
[statically determined][dynamic_dispatch]. As mentioned above, all that
needs to be done is to implement the `Error` trait:

```rust
trait Error: Debug + Display {
    fn description(&self) -> &str;
    fn cause(&self) -> Option<&Error>;
}
```

With this implementation, let's look at our most recent example. Note that
it is just as valid with the error type of `Box<Error>` as it was before
with `DoubleError`:

{boxing_errors.play}

### Смотрите также:

[Dynamic dispatch][dynamic_dispatch] and [`Error` trait][error]

[dynamic_dispatch]: http://doc.rust-lang.org/book/trait-objects.html#dynamic-dispatch
[error]: http://doc.rust-lang.org/std/error/trait.Error.html

We have seen that by implementing `Display` and `From` for our error type, we have enabled
usage of almost all of the std library error handling tools. That is, we missed one
capability: the ability to easily `box` our error type.

Namely, the std library will automatically convert from any type which implements the
`Error` trait into the trait object `Box<Error>` via `From`. To a library user, this
conveniently allows the following:

```rust
// Any error type automatically convertible to `Box<Error>` may be used here.
fn foo(...) -> Result<T, Box<Error>> { ... }
```

For example, a user may use a variety of libraries which each provide their own error
types. In order to define a valid `Result<T, E>` type, the user has a few choices:

* define a new wrapper error type around the external libraries error types
* convert it to `String` or some other intermediate choice
* box it up into `Box<Error>` via type erasure

Boxing it is a common choice. The only penalty is that the underlying error type is only known
at runtime and not [statically determined][dynamic_dispatch]. All that needs to be done to enable
this is implement the `Error` trait:

```rust
trait Error: Debug + Display {
    fn description(&self) -> &str;
    fn cause(&self) -> Option<&Error>;
}
```

By implementing this, our previous example would be just as valid when the error type
is `Box<Error>` as it was before with `DoubleError`.

{rethink.play}

### See also:

[Dynamic dispatch][dynamic_dispatch] and [`Error` trait][error]

[dynamic_dispatch]: http://doc.rust-lang.org/book/trait-objects.html#dynamic-dispatch
[error]: http://doc.rust-lang.org/std/error/trait.Error.html

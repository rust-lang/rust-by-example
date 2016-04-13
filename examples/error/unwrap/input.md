We determined a snake is an inappropriate gift for a princess. But what if 
she expected a gift and didn't receive one? That would be just as bad, so 
it needs to be handled! In the `std` library, an `enum` called `Option<T>` 
is used when absence is a possibility. It manifests itself as one of 
two "options":

* `Some(T)`: An element of type `T` was found
* `None`: No element was found

These can either be explicitly handled via `match` or implicitly with 
`unwrap`. Implicit handling either returns the inner element or `panic`s.

Note that it's possible to manually customize `panic` with 
[expect][expect], but `unwrap` otherwise leaves us with a less 
meaningful output than explicit handling. In the following example, 
explicit handling yields a more controlled result while retaining the 
option to `panic` if desired. 

{unwrap.play}

[expect]: http://doc.rust-lang.org/std/option/enum.Option.html#method.expect

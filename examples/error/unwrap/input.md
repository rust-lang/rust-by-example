We determined a snake is an inappropriate gift for a princess. What if she expected a gift
but did not receive one? Clearly that would be just as bad but how would it be handled? Well,
`Option<T>` is the type which is used when absense is a possibility. This manifests itself as
two choices:

* `Some<T>`: An element `T` was found
* `None`: No element was found

These can either be explicitly handled via `match` or implicitly with `unwrap`. `unwrap`,
deferring to the std library, either returns the inner element or `panics`. Regardless of
explicit or implicit handling, an `enum` such as `Option` will have all cases handled. The
compiler ensures that none are forgotten giving us more confidence in its robustness.

{unwrap.play}

As you can see, direct control yielded an even nicer result than the original `panic` along
with the choice to `panic` if we desired. `unwrap` on the other hand, deferring to the std
library left us with the most generic and unhelpful: "I unwrapped a `None`!" meanwhile yielding
the good results the rest of the time. A more meaningful message will require a better approach[^1].

[^1]: ignoring [expect][expect] which allows manual customization of the `panic` for now

[expect]: http://doc.rust-lang.org/std/option/enum.Option.html#method.expect

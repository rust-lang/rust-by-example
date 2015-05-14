Associated Items is an extension to `trait` generics which adds a different
feel and style to generics. It encompasses a set of rules pertaining to
[`item`s][items] of various types. The main one under under consideration here
is Associated Types. Associated Types adds one new concept:

* `trait`s can now internally define a `type` via the keyword `type`. This is
called an *output* type.

The result is much simpler usage patterns when the `trait` is generic over
containers.

### See also:

[RFC](
https://github.com/rust-lang/rfcs/blob/master/text/0195-associated-items.md
)
[items]: http://doc.rust-lang.org/reference.html#items

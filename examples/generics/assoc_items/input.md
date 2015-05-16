Associated Items is an extension to `trait` generics which encompasses a set
of rules pertaining to [`item`s][items] of various types. It allows `trait`s
to internally define a few new items including:

* `const`s via the keyword `const`.
* `type`s via the keyword `type`. This is called an *output* type.

Some advantages are:

* `trait`s now have access to `const`s.
* `type`s get simpler usage patterns when the `trait` is generic over
containers.

### See also:

[RFC](
https://github.com/rust-lang/rfcs/blob/master/text/0195-associated-items.md
)
[items]: http://doc.rust-lang.org/reference.html#items

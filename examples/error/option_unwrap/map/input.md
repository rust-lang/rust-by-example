`match` is a valid method for handling `Option`s. However, you may
eventually find heavy usage tedious, especially with operations only valid
with an input. In these cases, [combinators][combinators] can be used to
manage control flow in a modular fashion.

`Option` has a built in method called `map()`, a combinator for the simple
mapping of `Some -> Some` and `None -> None`. Multiple `map()` calls can be
chained together for even more flexibility.

In the following example, `process()` replaces all functions previous
to it while staying compact.

{map.play}

### Смотрите также:

[closures][closures], [`Option`][option], [`Option::map()`][map]

[combinators]: https://doc.rust-lang.org/book/glossary.html#combinators
[closures]: ../../fn/closures.html
[option]: http://doc.rust-lang.org/std/option/enum.Option.html
[map]: http://doc.rust-lang.org/std/option/enum.Option.html#method.map

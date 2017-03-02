`map()` was described as a chainable way to simplify `match` statements.
However, using `map()` on a function that returns an `Option<T>` results
in the nested `Option<Option<T>>`. Chaining multiple calls together can
then become confusing. That's where another combinator called `and_then()`,
known in some languages as flatmap, comes in.

`and_then()` calls its function input with the wrapped value and returns the result. 
If the `Option` is `None`, then it returns `None` instead.

In the following example, `cookable_v2()` results in an `Option<Food>`.
Using `map()` instead of `and_then()` would have given an
`Option<Option<Food>>`, which is an invalid type for `eat()`.

{and_then.play}

### Смотрите также:

[closures][closures], [`Option`][option], and [`Option::and_then()`][and_then]

[closures]: ../../fn/closures.html
[option]: http://doc.rust-lang.org/std/option/enum.Option.html
[and_then]: http://doc.rust-lang.org/std/option/enum.Option.html#method.and_then

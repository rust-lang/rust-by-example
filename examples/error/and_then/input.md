`map()` was described as a chainable way to simplify `match` statements. 
However, using `map()` on a function that returns an `Option<T>` results 
in the nested `Option<Option<T>>`. Chaining multiple calls together can 
then become confusing.

That's where `and_then()` comes in. Known in some languages as flatmap, `and_then()` calls its function input with the wrapped value or returns `None` if the `Option` is `None`.

In the following example, `cookable_v2()` results in an `Option<Food>`. 
Using `map()` instead of `and_then()` would have given an `Option<Option<Food>>`, 
which is an invalid type for `eat()`.

{and_then.play}

### See also:

[`Option`][option], [`Option::map()`][map], and [`Option::and_then()`][and_then]

[option]: http://doc.rust-lang.org/std/option/enum.Option.html
[map]: http://doc.rust-lang.org/std/option/enum.Option.html#method.map
[and_then]: http://doc.rust-lang.org/std/option/enum.Option.html#method.and_then 

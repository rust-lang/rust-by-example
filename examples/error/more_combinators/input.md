`map()` was previously described as a way to simplify a `match` which also allows chaining.
However, `map()` does not work for all cases because the constituents often occur in many
different combinations. Consider the following example:

{combinators.play}

The reason this worked is because `and_then()` happened to require the exact function type as an
input that was needed here. `map()` did not. Comparing the signatures of their input types, you
will see that when the function returned an `Option`, `and_then()` became the one and *only*
valid choice.

```rust
map():      FnOnce(T) -> U
and_then(): FnOnce(T) -> Option<U>
```

These are just two of many different combinators that are implemented on
[`Option`][option] by the std library for many different use cases. It is advantageous
to become familiar with them because they can simplify many error handling procedures and
avoid the ugly and suicidal `panic!()` alternative. The other common error handling type,
`Result`, also uses most of these same constructs so the skills are transferable.


### See also:

[`Option`][option], [`Option::map()`][map], and [`Option::and_then()`][and_then]

[option]: http://doc.rust-lang.org/std/option/enum.Option.html
[map]: http://doc.rust-lang.org/std/option/enum.Option.html#method.map
[and_then]: http://doc.rust-lang.org/std/option/enum.Option.html#method.and_then 

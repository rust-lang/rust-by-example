Notice in the previous example that our immediate reaction to calling
`parse` is to `map` the error from a library error into our new custom
error type:

```rust
.and_then(|s| s.parse::<i32>()
    .map_err(DoubleError::Parse)
```

Since this is a simple and common operation, it would be convenient if it
could be elided. Alas, because `and_then` is not sufficiently flexible, it
cannot. However, we can instead use `try!`.

`try!` was previously explained as either `unwrap` or `return Err(err)`.
This is only mostly true. It actually means `unwrap` or
`return Err(From::from(err))`. Since `From::from` is a conversion utility
between different types, this means that if you `try!` where the error is
convertible to the return type, it will convert automatically.

Here, we rewrite the previous example using `try!`. As a result, the
`map_err` will go away when `From::from` is implemented for our error type:

{reenter_try.play}

This is actually fairly clean now. Compared with the original `panic`, it
is very similar to replacing the `unwrap` calls with `try!` except that the
return types are `Result`. As a result, they must be destructured at the
top level.

Note that you should not expect error handling of this sort to always
replace `unwrap`. This type of error handling tripled our line count and
cannot really be considered simple (even when heavily biased by the small
code size).

Indeed, moving a 1000 line library from `unwrap` to more proper error
handling might be feasible in an additional 100 lines of code. However, the
necessary refactoring would most definitely not be trivial.

Many libraries might get away with only implementing `Display` and
adding `From` on an as needed basis. However, more serious libraries will
eventually need to meet higher expectations of error handling implementation.

### Смотрите также:

[`From::from`][from] and [`try!`][try]

[from]: http://doc.rust-lang.org/std/convert/trait.From.html
[try]: http://doc.rust-lang.org/std/macro.try!.html

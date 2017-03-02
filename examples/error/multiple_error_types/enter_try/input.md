Sometimes we just want the simplicity of `unwrap` without the possibility of
a `panic`. Until now, `unwrap` has forced us to nest deeper and deeper when
what we really wanted was to get the variable *out*. This is exactly the purpose of `try!`.

Upon finding an `Err`, there are two valid actions to take:

1. `panic!` which we already decided to try to avoid if possible
2. `return` because an `Err` means it cannot be handled

`try!` is *almost*[^1] exactly equivalent to an `unwrap` which `return`s
instead of `panic`s on `Err`s. Let's see how we can simplify the earlier
example that used combinators:

{try.play}

Note that up until now, we've been using `String`s as errors. However, they
are somewhat limiting as an error type. In the next section, we'll learn how
to make more structured and informative errors by defining their types.

[^1]: See [re-enter try!][re_enter_try] for more details.

### Смотрите также:

[`Result`][result] and [`io::Result`][io_result]

[result]: http://doc.rust-lang.org/std/result/enum.Result.html
[io_result]: http://doc.rust-lang.org/std/io/type.Result.html
[re_enter_try]: ../../error/reenter_try.html

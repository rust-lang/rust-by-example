The previous problem was awkward because avoiding `unwrap` forced us to nest deeper and
deeper when what we really wanted was to get the variable *out*. So, is there any way
to accomodate this approach without `panic`? Well, what is a valid action to take when
an `Err` is found? It turns out there are two:

1. `panic!` which we already decided to try to avoid if possible
2. `return` because an `Err` means it cannot be handled

This is exactly the purpose of `try!`; it is *almost*[^1] exactly equivalent to an
`unwrap` which `returns` instead of `panics` on `Errs`.

{try.play}

This really is a *huge* improvement but there is still the nagging issue of `map_err`. There is
actually a way to avoid it (we are using it everywhere it seems) but we are still missing some
details. First, we have to learn how to make better errors.

[^1]: See [re-enter try!][re_enter_try] for more details.

### See also:

[`Result`][result] and [`io::Result`][io_result]

[result]: http://doc.rust-lang.org/std/result/enum.Result.html
[io_result]: http://doc.rust-lang.org/std/io/type.Result.html
[re_enter_try]: /error/reenter_try.html

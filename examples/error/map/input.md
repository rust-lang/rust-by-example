Like before, `match` is a valid method for handling an `Option` however you may find that this
gets tedious with heavy usage of `Options`. Luckily, there is an easier way.

Consider the example below and in particular `peel()` and `chop()`; since each of these
operations is only valid when the input exists, it seems sensible that bad input simply means
the result is bad. This results in the simplistic mapping `Some -> Some` and `None -> None`.
This is actually so common that there is a built in method for it called `map()` which is
already implemented on `Option`.

The result of this is that `chop()` is simpler to write than any of the previous methods.
Furthermore, the ability to chain these together makes it even more flexible; `process()`
easily can replace all the previous functions and still be compact.

{map.play}

[option]: http://doc.rust-lang.org/std/option/enum.Option.html

### See also:

[`struct`s][structs]

[structs]: /custom_types/structs.html

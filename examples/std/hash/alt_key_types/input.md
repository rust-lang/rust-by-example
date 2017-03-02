Any type that implements the `Eq` and `Hash` traits can be a key in `HashMap`.
This includes:

* `bool` (though not very useful since there is only two possible keys)
* `int`, `uint`, and all variations thereof
* `String` and `&str` (protip: you can have a `HashMap` keyed by `String`
and call `.get()` with an `&str`)

Note that `f32` and `f64` do *not* implement `Hash`,
likely because [floating-point precision errors][floating]
would make using them as hashmap keys horribly error-prone.

All collection classes implement `Eq` and `Hash`
if their contained type also respectively implements `Eq` and `Hash`.
For example, `Vec<T>` will implement `Hash` if `T` implements `Hash`.

You can easily implement `Eq` and `Hash` for a custom type with just one line:
`#[derive(PartialEq, Eq, Hash)]`

The compiler will do the rest. If you want more control over the details,
you can implement `Eq` and/or `Hash` yourself.
This guide will not cover the specifics of implementing `Hash`.

To play around with using a `struct` in `HashMap`,
let's try making a very simple user logon system:

{alt_key_types.play}

[hash]: http://en.wikipedia.org/wiki/Hash_function
[floating]: http://en.wikipedia.org/wiki/Floating_point#Accuracy_problems

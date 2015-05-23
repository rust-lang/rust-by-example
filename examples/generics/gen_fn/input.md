The same set of rules can be applied to functions: a type `T` is only
generic if it has been preceded by `<T>`. This allows functions to have
a variety of forms, some which may have surprising consequences.

Generic function usage also sometimes requires explicitly specializing
the call. An explicitly specialized function call looks like:
`fun::<A, B, ...>()`.

{fn.play}

### See also:

[functions][fn] and [`struct`s][structs]

[fn]: /fn.html
[structs]: /custom_types/structs.html

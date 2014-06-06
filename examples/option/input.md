The `fail!` macro can be used to generate a runtime failure and start unwinding
the program. Upon unwinding, the runtime will take care of freeing all
the resources by calling the destructors of all the objects in scope.

Sometimes it is desirable to catch the failure of some parts of the program
instead of unwinding the whole program, this can be accomplished using the
`Option` enum (enums will be explained in detail in the next section).

`Option` can take two values: `None`, to indicate failure or lack of value, or
`Some`, a tuple struct that wraps a value. The type `T` of the wrapped value is
part of the type signature of option `Option<T>`.

{option.play}

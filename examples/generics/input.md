Generics is the topic of generalizing types and functionality to be more broad
than one specific type. This is extremely useful in reducing code duplication
in many ways. We will find though that being generic will involve taking
great care to actually specify what types a generic type is actually valid
over. This will require a rather involving syntax, though it seems
straightforward at first.

A type is specified as generic by `<A, B, ...>`. There are 2 basic rules
regarding this which are applied *at* the type's first use:

* Any type previously and locally specified to be generic is generic.
* Everything else is concrete (non-generic).

These rules play out like this:

{generics.play}

### See also:

[`struct`s][structs]

[structs]: /custom_types/structs.html

A `trait` that is generic over its container type has type specification
requirements - users of the `trait` *must* specify all of its generic types.

In the example below, the `Contains` `trait` allows the use of the generic
types `A` and `B`. The trait is then implemented for the `Container` type,
specifying `i32` for `A` and `B` so that it can be used with `fn difference()`.

Because `Contains` is generic, we are forced to explicitly state *all* of the
generic types for `fn difference()`. In practice, we want a way to express that
`A` and `B` are determined by the *input* `C`. As you will see in the next
section, associated types provide exactly that capability.

{problem.play}

### Смотрите также:

[`struct`s][structs], and [`trait`s][traits]

[structs]: ../../custom_types/structs.html
[traits]: ../../trait.html

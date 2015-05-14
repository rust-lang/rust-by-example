`trait`s generic over container types have strict type specification
requirements. Users of the `trait`s *must* specify all generic types, even
irrelevant ones.

Specifically, examine the `Contains` `trait` and the `difference()` function
which utilizes it. The fact that `Contains` is generic immediately forces
users of the `trait` regardless of need to explicitly state *all* the
`trait`'s generic types.

{problem.play}

The problem is we require a way to express that `A` and `B` are determined
by the *input* `C`. Having to express them as *input* parameters is just
hindering. Associated types provides exactly that capability.

### See also:

[`struct`s][structs], and [`trait`s][traits]


[structs]: /custom_types/structs.html
[traits]: /trait.html

Any type that implements the `Eq` and `Hash` traits can be a key in `HashMap`. 
This includes:

* `bool` (though not very useful since there is only two possible keys)
* `int`, `uint`, and all variations thereof 
(see `SmallIntMap` for a more streamlined map implementation keyed by `uint`)

Note that `f32` and `f64` do *not* implement `Hash`, 
likely because floating-point precision errors 
would make using them as hash map keys very frustrating.

All collection classes implement `Eq`, and implicitly implement `Hash` 
if their contained type also implements `Hash`. 
E.g. `Vec<T>` will implement `Hash` if `T` implements `Hash`.

You can easily implement `Eq` and `Hash` for a custom type with just one line: 
`#[deriving(Eq,Hash)]`

The compiler will do the rest. If you want more control over the details, 
you can implement `Eq` and `Hash` yourself. 
This guide will not cover the specifics of implementing `Hash`. 

Note that two instances of a type that are equal 
(such that `a == b` returns `true`) should hash to the same value, 
but two instances with the same hash don't necessarily have to be equal. 
Read up on [hashes][hash] and [hash collisions][collision] 
for more information.

To play around with using a `struct` in `HashMap`, 
let's try making a very simple user logon system:

{alt-key-types.play}

[hash]: http://en.wikipedia.org/wiki/Hash_function
[collision]: http://en.wikipedia.org/wiki/Hash_collision

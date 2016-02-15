A `trait` is a collection of methods defined for an unknown type:
`Self`. They can access other methods declared in the same trait.

Traits can be implemented for any data type. In the example below,
we define `Animal`, a group of methods. The `Animal` `trait` is 
then implemented for the `Sheep` data type, allowing the use of 
methods from `Animal` with a `Sheep`.

{trait.play}

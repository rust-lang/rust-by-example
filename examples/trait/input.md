A `trait` is a collection of methods defined for an unknown type. A trait
can be implemented for any data type. When some instances implement the same
trait, they can be grouped in an array since they provide the same interface.

{trait.rs}

{trait.out}

The compiler is capable of providing basic implementations for some traits via
the `#[deriving]` attribute. These traits can still be manually implemented if
a more complex behavior is required.

{deriving.rs}

{deriving.out}

This is a list of the "derivable" traits:
* Comparison traits:
  [`Eq`](http://static.rust-lang.org/doc/master/std/cmp/trait.Eq.html),
  [`TotalEq`](http://static.rust-lang.org/doc/master/std/cmp/trait.TotalEq.html),
  [`Ord`](http://static.rust-lang.org/doc/master/std/cmp/trait.Ord.html),
  [`TotalOrd`](http://static.rust-lang.org/doc/master/std/cmp/trait.TotalOrd.html)
* Serialization:
  [`Encodable`](http://static.rust-lang.org/doc/master/serialize/trait.Encodable.html),
  [`Decodable`](http://static.rust-lang.org/doc/master/serialize/trait.Decodable.html)
* [`Clone`](http://static.rust-lang.org/doc/master/std/clone/trait.Clone.html),
  to create `T` from `&T` via a copy.
* [`Hash`](http://static.rust-lang.org/doc/master/std/hash/trait.Hash.html), to
  compute a hash from `&T`.
* [`Rand`](http://static.rust-lang.org/doc/master/rand/trait.Rand.html), to
  create a random instance of a data type.
* [`Default`](http://static.rust-lang.org/doc/master/std/default/trait.Default.html),
  to create an empty instance of a data type.
* [`Zero`](http://static.rust-lang.org/doc/master/std/num/trait.Zero.html), to
  create a zero instance of a numeric data type.
* [`FromPrimitive`](http://static.rust-lang.org/doc/master/std/num/trait.FromPrimitive.html),
  to create an instance from a numeric primitive.
* [`Show`](http://static.rust-lang.org/doc/master/std/fmt/trait.Show.html), to
  format a value using the `{}` formatter.

The compiler is capable of providing basic implementations for some traits via
the `#[deriving]` [attribute](/attribute.html). These traits can still be
manually implemented if a more complex behavior is required.

{deriving.play}

This is a list of the "derivable" traits:
* Comparison traits:
  [`Eq`](http://doc.rust-lang.org/std/cmp/trait.Eq.html),
  [`PartialEq`](http://doc.rust-lang.org/std/cmp/trait.PartialEq.html),
  [`Ord`](http://doc.rust-lang.org/std/cmp/trait.Ord.html),
  [`PartialOrd`](http://doc.rust-lang.org/std/cmp/trait.PartialOrd.html)
* Serialization:
  [`Encodable`](http://doc.rust-lang.org/serialize/trait.Encodable.html),
  [`Decodable`](http://doc.rust-lang.org/serialize/trait.Decodable.html)
* [`Clone`](http://doc.rust-lang.org/std/clone/trait.Clone.html),
  to create `T` from `&T` via a copy.
* [`Hash`](http://doc.rust-lang.org/std/hash/trait.Hash.html), to
  compute a hash from `&T`.
* [`Rand`](http://doc.rust-lang.org/rand/trait.Rand.html), to
  create a random instance of a data type.
* [`Default`](http://doc.rust-lang.org/std/default/trait.Default.html),
  to create an empty instance of a data type.
* [`Zero`](http://doc.rust-lang.org/std/num/trait.Zero.html), to
  create a zero instance of a numeric data type.
* [`FromPrimitive`](http://doc.rust-lang.org/std/num/trait.FromPrimitive.html),
  to create an instance from a numeric primitive.
* [`Show`](http://doc.rust-lang.org/std/fmt/trait.Show.html), to
  format a value using the `{}` formatter.

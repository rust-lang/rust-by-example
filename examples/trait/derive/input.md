The compiler is capable of providing basic implementations for some traits via
the `#[derive]` [attribute][attribute]. These traits can still be
manually implemented if a more complex behavior is required.

{derive.play}

This is a list of the "derivable" traits:
* Comparison traits:
  [`Eq`][eq],
  [`PartialEq`][partial-eq],
  [`Ord`][ord],
  [`PartialOrd`][partial-ord]
* Serialization:
  [`Encodable`][encodable],
  [`Decodable`][decodable]
* [`Clone`][clone],
  to create `T` from `&T` via a copy.
* [`Hash`][hash], to
  compute a hash from `&T`.
* [`Rand`][rand], to
  create a random instance of a data type.
* [`Default`][default],
  to create an empty instance of a data type.
* [`Zero`][zero], to
  create a zero instance of a numeric data type.
* [`FromPrimitive`][from-primitive],
  to create an instance from a numeric primitive.
* [`Show`][show], to
  format a value using the `{:?}` formatter.

[attribute]: /attribute.html
[eq]: http://doc.rust-lang.org/std/cmp/trait.Eq.html
[partial-eq]: http://doc.rust-lang.org/std/cmp/trait.PartialEq.html
[ord]: http://doc.rust-lang.org/std/cmp/trait.Ord.html
[partial-ord]: http://doc.rust-lang.org/std/cmp/trait.PartialOrd.html
[encodable]: http://doc.rust-lang.org/serialize/trait.Encodable.html
[decodable]: http://doc.rust-lang.org/serialize/trait.Decodable.html
[clone]: http://doc.rust-lang.org/std/clone/trait.Clone.html
[hash]: http://doc.rust-lang.org/std/hash/trait.Hash.html
[rand]: http://doc.rust-lang.org/rand/trait.Rand.html
[default]: http://doc.rust-lang.org/std/default/trait.Default.html
[zero]: http://doc.rust-lang.org/std/num/trait.Zero.html
[from-primitive]: http://doc.rust-lang.org/std/num/trait.FromPrimitive.html
[show]: http://doc.rust-lang.org/std/fmt/trait.Show.html

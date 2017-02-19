The compiler is capable of providing basic implementations for some traits via
the `#[derive]` [attribute][attribute]. These traits can still be
manually implemented if a more complex behavior is required.

The following is a list of derivable traits:
* Comparison traits:
  [`Eq`][eq], [`PartialEq`][partial-eq], [`Ord`][ord], [`PartialOrd`][partial-ord]
* [`Clone`][clone], to create `T` from `&T` via a copy.
* [`Copy`][copy], to give a type 'copy semantics' instead of 'move semantics'
* [`Hash`][hash], to compute a hash from `&T`.
* [`Default`][default], to create an empty instance of a data type.
* `Zero`, to create a zero instance of a numeric data type.
* [`Debug`][debug], to format a value using the `{:?}` formatter.
 
{derive.play}

### See also:
[`derive`][derive]

[attribute]: /attribute.html
[eq]: http://doc.rust-lang.org/std/cmp/trait.Eq.html
[partial-eq]: http://doc.rust-lang.org/std/cmp/trait.PartialEq.html
[ord]: http://doc.rust-lang.org/std/cmp/trait.Ord.html
[partial-ord]: http://doc.rust-lang.org/std/cmp/trait.PartialOrd.html
[clone]: http://doc.rust-lang.org/std/clone/trait.Clone.html
[copy]: https://doc.rust-lang.org/core/marker/trait.Copy.html
[hash]: http://doc.rust-lang.org/std/hash/trait.Hash.html
[default]: http://doc.rust-lang.org/std/default/trait.Default.html
[debug]: http://doc.rust-lang.org/std/fmt/trait.Debug.html
[derive]: https://doc.rust-lang.org/reference.html#derive

Consider a `HashSet` as a `HashMap` where we just care about the keys (
`HashSet<T>` is, in actuality, just a wrapper around `HashMap<T, ()>`).

"What's the point of that?" you ask. "I could just store the keys in a `Vec`."

A `HashSet`'s unique feature is that
it is guaranteed to not have duplicate elements.
That's the contract that any set collection fulfills.
`HashSet` is just one implementation. (see also: [`BTreeSet`][treeset])

If you insert a value that is already present in the `HashSet`,
(i.e. the new value is equal to the existing and they both have the same hash),
then the new value will replace the old.

This is great for when you never want more than one of something,
or when you want to know if you've already got something.

But sets can do more than that.

Sets have 4 primary operations (all of the following calls return an iterator):

* `union`: get all the unique elements in both sets.

* `difference`: get all the elements that are in the first set but not the second.

* `intersection`: get all the elements that are only in *both* sets.

* `symmetric_difference`:
get all the elements that are in one set or the other, but *not* both.

Try all of these in the following example.

{hashset.play}

(Examples adapted from the [documentation.][hash-set])

[treeset]: http://doc.rust-lang.org/std/collections/struct.BTreeSet.html
[hash-set]: http://doc.rust-lang.org/std/collections/struct.HashSet.html#method.difference

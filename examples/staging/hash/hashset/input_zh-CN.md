Consider a `HashSet` as a `HashMap` where the key and value are the same. 
Or, to be more precise, a `HashMap` where we just care about the keys.

"What's the point of that?" you ask. "I could just store the keys in a `Vec`."

A `HashSet`'s unique feature is that 
it is guaranteed to not have duplicate elements. 
That's the contract that any [`Set`][set] implementation fulfills. 
`HashSet` is just one implementation. (see also: [`MutableSet`][mutableset])

If you insert a value that is already present in the `HashSet`, 
(i.e. the new value is equal to the existing and they both have the same hash), 
then the new value will replace the old.

This is great for when you never want more than one of something, 
or when you want to know if you've already got something.

But sets can do more than that. 
If they weren't important, then why would there be a 
[branch of mathematics][set-theory] dedicated to them? 

Sets have 4 primary operations (all of the following calls return an iterator):

* `union`: get all the elements in one set or the other.

* `difference`: get all the elements that are in the first set but not the second.

* `intersection`: get all the elements that are only in *both* sets.

* `symmetric_difference`: 
get all the elements that are in one set or the other, but *not* both.

Try all of these in the following example.

{hashset.play}

Sets don't seem so pointless now, do they? 

(Examples adapted from the [documentation.][hash-set])

[set]: http://doc.rust-lang.org/std/collections/trait.Set.html
[mutableset]: http://doc.rust-lang.org/std/collections/trait.MutableSet.html
[set-theory]: http://en.wikipedia.org/wiki/Set_theory
[hash-set]: http://doc.rust-lang.org/std/collections/hashmap/struct.HashSet.html#method.difference

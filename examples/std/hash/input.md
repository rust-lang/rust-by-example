Where vectors store values by an integer index, `HashMap`s store values by key.
`HashMap` keys can be booleans, integers, strings,
or any other type that implements the `Eq` and `Hash` traits.
More on this in the next section.

Like vectors, `HashMap`s are growable, but HashMaps can also shrink themselves
when they have excess space.
You can create a HashMap with a certain starting capacity using
`HashMap::with_capacity(uint)`, or use `HashMap::new()` to get a HashMap
with a default initial capacity (recommended).

{hash.play}

For more information on how hashing and hash maps
(sometimes called hash tables) work, have a look at
[Hash Table Wikipedia][wiki-hash]

[wiki-hash]: http://en.wikipedia.org/wiki/Hash_table

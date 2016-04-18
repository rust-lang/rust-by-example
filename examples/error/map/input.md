`match` is a valid method for handling `Option`s. However, you may eventually
find heavy usage tedious; this is the case especially with operations that
are only valid with an input.

For situations where a simplistic mapping of `Some -> Some` and 
`None -> None` is needed, `Option` has a built in method called `map()`.

Multiple `map()` calls can be chained together for even more flexibility.
In the following example, `process()` easily replaces all functions previous
to it while staying compact.

{map.play}

### See also:

[closures][closures]

[closures]: /fn/closures.html

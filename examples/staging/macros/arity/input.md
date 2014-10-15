Macros can have different implementations depending on their arity (number of
arguments), this can be used to implement some form of function overloading.
Here we implement a macro similar to Python's overloaded
[range](https://docs.python.org/3/library/stdtypes.html?highlight=range#range)
function:

```rust
// Here is how it should work
range!(10i)       // Returns 0..9
range!(3i, 10)    // Returns 3..9
range!(3i, 10, 2) // Returns 3, 5, 7, 9 (3..9 in increments of 2)
```

{range.play}


An array is a collection of objects of the same type `T`, stored in contiguous
memory. Arrays are created using brackets `[]`, and their size, which is known
at compile time, is part of their type signature `[T; size]`.

Slices are similar to arrays, but their size is not known at compile time.
Instead, a slice is a two-word object, the first word is a pointer to the data,
and the second word is the length of the slice. Slices can be used to borrow a
section of an array, and have the type signature `&[T]`.

{array.play}

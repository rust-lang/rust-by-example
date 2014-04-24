Arrays are collections of items of the same type. Arrays are declared using
brackets `[]`, and their size is part of their type signature `[T, ..size]`.

Slices are similar to arrays, but their size is not known at compile time,
instead a slice is two word object, the first word is a pointer to the data
and the second word is the length of the slice.

{array.rs}

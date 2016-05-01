A tuple is a collection of values of different types. Tuples are constructed
using parentheses `()`, and each tuple itself is a value with type signature
`(T1, T2, ...)`, where `T1`, `T2` are the types of its members. Functions can
use tuples to return multiple values, as tuples can hold any number of values.

{tuples.play}

### Activity

 1. *Recap*: Add the `fmt::Display` trait to the Matrix `struct` in the above example,
    so that if you switch from printing the debug format `{:?}` to the display
    format `{}`, you see the following output:
```
( 1.1 1.2 )
( 2.1 2.2 )
```
    You may want to refer back to the example for [print display][print_display].
 2. Add a `transpose` function using the `reverse` function as a template, which
    accepts a matrix as an argument, and returns a matrix in which two elements
    have been swapped. For example:
```
println!("Matrix:\n{}", matrix);
println!("Transpose:\n{}", transpose(matrix));
```
results in the output:
```
Matrix:
( 1.1 1.2 )
( 2.1 2.2 )
Transpose:
( 1.1 2.1 )
( 1.2 2.2 )
```

[print_display]: /hello/print/print_display.html

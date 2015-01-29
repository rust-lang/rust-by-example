In Rust, almost every statement is an expression, meaning that the statement
returns a value. This may not always be desired, so the output can be
suppressed by ending the expression with a semicolon `;`.

Blocks are expressions too, so they can be used as
[r-values][rvalue]
in assignments. The last expression in the block will be assigned to the
[l-value][lvalue].
However, if the last expression of the block ends with a semicolon, the
return value will be `()`.

{expression.play}

[rvalue]: https://en.wikipedia.org/wiki/Value_%28computer_science%29#lrvalue
[lvalue]: https://en.wikipedia.org/wiki/Value_%28computer_science%29#lrvalue

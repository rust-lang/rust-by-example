Several statements (like if-else) in Rust are actually expressions, i.e. they
return a value. Since Rust is strongly typed, this means each branch of an
expression must return the same type.

Sometimes is desirable to ignore the output of an expression, this can be done
by terminating the expression with a semicolon `;`. Expressions terminated with
a semicolon will return the unit type `()`. Empty if-else branches will also
return this type.

{expression.rs}

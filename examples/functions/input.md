Functions in Rust are declared using the `fn` keyword, arguments are type
annotated just like variables and if the function returns a value, the return
type must be specified after an arrow `->`.

The final expression in the function will be used as return value, unless is
terminated by a semicolon. Alternatively, the `return` keyword can be used to
return a value earlier from the function, even from inside loops or ifs.

Let's rewrite fizzbuzz using functions!

{functions.rs}

{functions.play}

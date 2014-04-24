Functions in Rust are declared using the `fn` keyword, arguments are type
annotated just like variables and if the function returns a value, the return
type must be specified after an arrow `->`.

The final expression in the function will be used as return value,
alternatively the `return` keyword can be used to return a value early from the
function.

Let's rewrite fizzbuzz using functions!

{functions.rs}

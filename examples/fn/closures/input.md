Closures in Rust, also called lambdas, are functions that can capture 
the enclosing environment. For example, a closure that captures the x 
variable:
```Rust
|val| val + x
```

The syntax and capabilities of closures make them very convenient for 
on the fly usage. Calling a closure is exactly like calling a function.
However, both input and return types *can* be inferred and input 
variable names *must* be specified.

Other characteristics of closures include:
* using `||` instead of `()` around input variables.
* optional body delimination (`{}`) for a single expression (mandatory otherwise).
* the ability to capture the outer environment variables.

{closures.play}

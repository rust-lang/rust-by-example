Closures in Rust, also called lambdas, are functions 
that can capture the enclosing environment. Their syntax and capabilities make them
very convenient for on the fly usage. Some characteristics include:

* uses `||` instead of `()` around input variables.
* *both* input and return *types* can be inferred.
* input variable *names* must be specified.
* body delimination (`{}`) is optional for a single expression. Mandatory
otherwise.
* the outer environment variables *may* be captured.
* calling a closure is exactly like a function: `call(var)`.

{closures.play}

Closures[^*] in Rust are functions with a slightly specialized syntax which
can capture the enclosing environment. Their syntax and capabilities make them
very convenient for on the fly usage. Some characteristics include:

* uses `||` instead of `()` around input variables.
* *both* input and return *types* can be inferred.
* input variable *names* must be specified.
* body delimination (`{}`) is optional for a single expression. Mandatory
otherwise.
* the outer environment variables *may* be captured.
* calling a closure is exactly like a function: `call(var)`.

{closures.play}

[^*]: Also called `lambdas` or `anonymous functions`.

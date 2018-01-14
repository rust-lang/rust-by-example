# Closures

Closures in Rust, also called lambda expressions or lambdas, are functions that can capture 
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

```rust,editable
fn main() {
    // Increment via closures and functions.
    fn  function            (i: i32) -> i32 { i + 1 }

    // Closures are anonymous, here we are binding them to references
    // Annotation is identical to function annotation but is optional
    // as are the `{}` wrapping the body. These nameless functions
    // are assigned to appropriately named variables.
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred  = |i     |          i + 1  ;

    let i = 1;
    // Call the function and closures.
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred.
    let one = || 1;
    println!("closure returning one: {}", one());

}
```

# Closures

Closures are functions that can capture the enclosing environment. For example,
a closure that captures the `x` variable:

```Rust
|val| val + x
```

The syntax and capabilities of closures make them very convenient for on the fly
usage. Calling a closure is exactly like calling a function. However, both input
and return types *can* be inferred and input variable names *must* be specified.

Other characteristics of closures include:

- using `||` instead of `()` around input variables.
- optional body delimitation (`{}`) for a single line expression (mandatory
  otherwise).
- the ability to capture the outer environment variables.

```rust,editable
fn main() {
    let outer_var = 42;

    // A regular function can't refer to variables in the enclosing environment
    //fn function(i: i32) -> i32 { i + outer_var }
    // TODO: uncomment the line above and see the compiler error. The compiler
    // suggests that we define a closure instead.

    // Closures are anonymous, here we are binding them to references.
    // Annotation is identical to function annotation but is optional
    // as are the `{}` wrapping the body. These nameless functions
    // are assigned to appropriately named variables.
    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    let closure_inferred = |i| i + outer_var;

    // Call the closures.
    println!("closure_annotated: {}", closure_annotated(1));
    println!("closure_inferred: {}", closure_inferred(1));
    // Once closure's type has been inferred, it cannot be inferred again with another type.
    //println!("cannot reuse closure_inferred with another type: {}", closure_inferred(42i64));
    // TODO: uncomment the line above and see the compiler error.

    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred.
    let one = || 1;
    println!("closure returning one: {}", one());
}
```

# Scope and Shadowing

Variable bindings have a scope, and are constrained to live in a *block*. A
block is a collection of statements enclosed by braces `{}`. Also, [variable
shadowing][variable-shadow] is allowed.

```rust,editable,ignore,mdbook-runnable
fn main() {
    // This binding lives in the main function
    let long_lived_binding = 1;

    // This is a block, and has a smaller scope than the main function
    {
        // This binding only exists in this block
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);

        // This binding *shadows* the outer one
        let long_lived_binding = 5_f32;

        println!("inner long: {}", long_lived_binding);
    }
    // End of the block

    // Error! `short_lived_binding` doesn't exist in this scope
    println!("outer short: {}", short_lived_binding);
    // FIXME ^ Comment out this line

    println!("outer long: {}", long_lived_binding);
    
    // This binding also *shadows* the previous binding
    let long_lived_binding = 'a';
    
    println!("outer long: {}", long_lived_binding);
}
```

[variable-shadow]: https://en.wikipedia.org/wiki/Variable_shadowing

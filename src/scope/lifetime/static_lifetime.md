# Static

A `'static` lifetime is the longest possible lifetime, and lasts for 
the lifetime of the running program. A `'static` lifetime may also be 
coerced to a shorter lifetime. There are two ways to make a variable 
with `'static` lifetime, and both are stored in the read-only memory
of the binary:

* Make a constant with the `static` declaration.
* Make a `string` literal which has type: `&'static str`.

See the following example for a display of each method:

```rust,editable
// Make a constant with `'static` lifetime.
static NUM: i32 = 18;

// Returns a reference to `NUM` where its `'static` 
// lifetime is coerced to that of the input argument.
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn main() {
    {
        // Make a `string` literal and print it:
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);

        // When `static_string` goes out of scope, the reference
        // can no longer be used, but the data remains in the binary.
    }
    
    {
        // Make an integer to use for `coerce_static`:
        let lifetime_num = 9;

        // Coerce `NUM` to lifetime of `lifetime_num`:
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }
    
    println!("NUM: {} stays accessible!", NUM);
}
```

### See also:

[`'static` constants][static_const]

[static_const]: ../../custom_types/constants.md
# Static

`'static` is one of the reserved lifetime specifiers. It has a special meaning
in Rust. A __reference__ that has a `'static` lifetime is guaranteed to be 
valid for the entire lifetime of the running program. 

There are two ways to create data that lives for the entire lifetime of
a program and both are stored in the read-only memory of the binary:

* Make a constant with the `static` declaration.
* Make a `string` literal which has type: `&'static str`.

__Warning__: It's easy to get confused about `'static` because all variable
bindings that own their data, eg. do not contain references, fulfill a 
bound on `'static`:

```rust,editable,compile_fail
use std::fmt::Debug;

fn print_it( input: impl Debug + 'static )
{
    println!( "'static value passed in is: {:?}", input );
}

fn use_it()
{
    let i = 5;     // i is owned and contains no references, thus it's 'static
    print_it( i ); // valid call
    
    // oops, &i only has the lifetime defined by the scope of 
    // use_it(), so it's not 'static
    print_it( &i ); 
}
```


The compiler will tell you:
```ignore
error[E0597]: `i` does not live long enough
  --> src/lib.rs:15:15
   |
15 |     print_it( &i ); 
   |     ----------^^--
   |     |         |
   |     |         borrowed value does not live long enough
   |     argument requires that `i` is borrowed for `'static`
16 | }
   | - `i` dropped here while still borrowed
```

A bound on `'static` means that the callee does not have to worry
about their variable binding becoming invalid after some time.
They can use it as long as they want. In contrast to a `'static` 
__reference__, this is unrelated to the runtime of the entire program.

A `'static` lifetime may also be coerced to a shorter lifetime. 

See the following example for uses of `'static`:

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

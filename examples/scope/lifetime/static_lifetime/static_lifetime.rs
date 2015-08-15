static NUM: i32 = 18;

// Return a reference to `NUM` which is coerced to the
// lifetime of `'a` which was used as an input.
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn main() {
    {
        // String literals are references to read-only memory
        let static_string = "In read-only memory";

        // When `_static_string` goes out of scope, we can no longer refer to
        // the underlying data, but the string remains in the read-only memory
        println!("static_string holds: {}", static_string);
    }
    
    println!("but now it's gone.");
    println!("NUM: {} is still around though!", NUM);

    {
        let i = 9;
        let coerced_num = coerce_static(&i);

        println!("coerced_num: {}", coerced_num);
    }
}

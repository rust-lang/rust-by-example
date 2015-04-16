static NUM: i32 = 18;

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
}

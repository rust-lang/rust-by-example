fn main() {
    // Start with two regular values
    let value = 5;
    let mut mut_value = 6;

    // To destructure into `&5` (reference to 5), use the `ref` keyword.
    match value {
        // `println!` can handle both regular values and references
        // so it doesn't care which we give it. `r` will have type `&i32`.
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // Likewise, to get a mutable reference `&mut 6` back,
    // `ref mut` is used.
    match mut_value {
        ref mut m => {
            // Got a reference. Gotta dereference it before we can
            // add anything to it.
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }
}

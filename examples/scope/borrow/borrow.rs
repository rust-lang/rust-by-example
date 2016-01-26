// This function takes ownership of a box and destroys it 
fn eat_box(boxed_int: Box<i32>) {
    println!("Destroying box that contains {}", boxed_int);
}

// This function borrows an i32
fn borrow_box(borrowed_int: &i32) {
    println!("This int is: {}", borrowed_int);
}

fn main() {
    // Create a boxed integer
    let boxed_int = Box::new(5);

    // Borrow the contents of the box. Ownership is not taken,
    // so the contents can be borrowed again.
    borrow_box(&boxed_int);
    borrow_box(&boxed_int);

    {
        // Take a reference to the data contained inside the box
        let _ref_to_int: &i32 = &boxed_int;

        // Error! 
        // Can't destroy `boxed_int` while the inner value is borrowed.
        eat_box(boxed_int);
        // FIXME ^ Comment out this line

        // `_ref_to_int` goes out of scope and is no longer borrowed.
    }

    // Box can now give up ownership to `eat_box` and be destroyed
    eat_box(boxed_int);
}

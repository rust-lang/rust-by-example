// This function takes ownership of the box
fn eat_box(boxed_int: Box<i32>) {
    println!("destroying box that contains {}", boxed_int);
}

// This function borrows the box
fn peep_inside_box(borrowed_box: &Box<i32>) {
    println!("This box contains {}", borrowed_box);
}

fn main() {
    // A boxed integer
    let boxed_int = Box::new(5);

    // Borrow the box, ownership is not taken
    peep_inside_box(&boxed_int);

    // The box can be borrowed again
    peep_inside_box(&boxed_int);

    {
        // Take a reference to the data contained inside the box
        let _ref_to_int: &i32 = &*boxed_int;

        // Error! Can't destroy boxed_int, while the inner value has been
        // borrowed
        eat_box(boxed_int);
        // FIXME ^ Comment out this line

        // `_ref_to_int` goes out of scope
    }

    // Give up ownership of the box
    eat_box(boxed_int);
}

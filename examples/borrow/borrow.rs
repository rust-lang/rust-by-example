use std::owned::Box;

// this function takes ownership of the box
fn eat_box(boxed_int: Box<int>) {
    // the box gets destroyed in this scope
}

// this function borrows the box
fn peep_inside_box(borrowed_box: &Box<int>) {
    println!("This box contains {}", borrowed_box);
}

fn main() {
    // boxed integer
    let boxed_int = box 5;

    // borrow the box, ownership is not taken
    peep_inside_box(&boxed_int);

    // give up ownership of the box
    eat_box(boxed_int);
}

use std::owned::Box;

// this function takes ownership of the heap allocated memory
fn destroy_box(boxed_int: Box<int>) {
    // boxed_int will be destroyed in this scope, and the memory will be
    // freed
}

fn main() {
    // heap allocated integer
    let boxed_int = box 5;

    // boxed_int loses ownership of the heap allocated integer
    destroy_box(boxed_int);

    // Error: this would dereference freed memory
    // the compiler forbids it, because the resource has been moved
    let invalid_dereference = *boxed_int;
}

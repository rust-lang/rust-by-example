use std::mem::size_of_val;

struct Point {
    x: f64,
    y: f64,
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn boxed_origin() -> ~Point {
    // heap allocate this point, and return a pointer to it
    ~Point { x: 0.0, y: 0.0 }
}

fn main() {
    // stack allocated variables
    let point = origin();
    let rectangle = Rectangle {
        p1: origin(),
        p2: Point { x: 3.0, y: 4.0 }
    };

    // heap allocated rectangle
    let boxed_rectangle = ~Rectangle { p1: origin(), p2: origin() };

    // function output can be boxed
    let boxed_point = ~origin();

    // double indirection
    let box_in_a_box: ~~Point = ~boxed_origin();

    println!("Point occupies {} bytes in the stack",
             size_of_val(&point));
    // The `&` means that we are passing to the function, a reference to
    // the object rather than a copy of it. In Rust, this is known as
    // *borrowing* and will be covered in detail in its own section.
    println!("Rectangle occupies {} bytes in the stack",
             size_of_val(&rectangle));

    // box size = pointer size
    println!("Boxed point occupies {} bytes in the stack",
             size_of_val(&boxed_point));
    println!("Boxed rectangle occupies {} bytes in the stack",
             size_of_val(&boxed_rectangle));

    // bring boxed point data back into the stack (i.e. unbox)
    let unboxed_point = *boxed_point;
    println!("Unboxed point occupies {} bytes in the stack",
             size_of_val(&unboxed_point));
}

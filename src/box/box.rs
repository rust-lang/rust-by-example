use std::mem::size_of_val;
use std::owned::Box;

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

fn boxed_origin() -> Box<Point> {
    // heap allocate this point, and return a pointer to it
    box Point { x: 0.0, y: 0.0 }
}

fn main() {
    // all type annotations are superfluous
    // stack allocated variables
    let point: Point = origin();
    let rectangle: Rectangle = Rectangle {
        p1: origin(),
        p2: Point { x: 3.0, y: 4.0 }
    };

    // heap allocated rectangle
    let boxed_rectangle: Box<Rectangle> = box Rectangle {
        p1: origin(),
        p2: origin()
    };

    // function output can be boxed
    let boxed_point: Box<Point> = box origin();

    // double indirection
    let box_in_a_box: Box<Box<Point>> = box boxed_origin();

    println!("Point occupies {} bytes in the stack",
             size_of_val(&point));
    // The `&` means that we are passing the object by reference, rather
    // than by value. In Rust, this is known as *borrowing* and will be
    // covered in detail in its own section.
    println!("Rectangle occupies {} bytes in the stack",
             size_of_val(&rectangle));

    // box size = pointer size
    println!("Boxed point occupies {} bytes in the stack",
             size_of_val(&boxed_point));
    println!("Boxed rectangle occupies {} bytes in the stack",
             size_of_val(&boxed_rectangle));

    // copy data of boxed_point into the stack
    let unboxed_point: Point = *boxed_point;
    println!("Unboxed point occupies {} bytes in the stack",
             size_of_val(&unboxed_point));

    // unboxing via deconstruction
    let box another_unboxed_point = boxed_point;
    println!("Another unboxed point occupies {} bytes in the stack",
             size_of_val(&unboxed_point));
}

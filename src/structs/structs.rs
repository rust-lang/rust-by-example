// unit struct
struct Nil;

// tuple struct
struct Pair(int, f64);

// a struct with two fields
struct Point {
    x: f64,
    y: f64,
}

// reusing structs as fields
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn main() {
    // instantiate a Point
    let point: Point = Point { x: 3.0, y: 4.0 };

    // access fields
    println!("Point coordinates: ({}, {})", point.x, point.y);

    // destructuring using let
    let Point { x: my_x, y: my_y } = point;

    let rectangle = Rectangle {
        // structs are expressions too
        p1: Point { x: my_y, y: my_x },
        p2: point
    };

    // instantiate a unit struct
    let nil = Nil;

    // instantiate a tuple struct
    let pair = Pair(1, 0.0);

    // destructure a tuple struct
    let Pair(integer, decimal) = pair;
}

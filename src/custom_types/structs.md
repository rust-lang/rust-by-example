# Structures

There are three types of structures ("structs") that can be created using the
`struct` keyword:

* Tuple structs, which are, basically, named tuples.
* The classic [C structs][c_struct]
* Unit structs, which are field-less, are useful for generics.

```rust,editable
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

// A unit struct
struct Nil;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn main() {
    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);


    // Instantiate a `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our other one
    let new_point = Point { x: 0.1, ..point };
    // `new_point.y` will be the same as `point.y` because we used that field from `point`
    println!("second point: ({}, {})", new_point.x, new_point.y);

    // Destructure the point using a `let` binding
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    // Instantiate a unit struct
    let _nil = Nil;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}
```

### Activity

1. Add a function `rect_area` which calculates the area of a rectangle (try 
   using nested destructuring). 
2. Add a function `square` which takes a `Point` and a `f32` as arguments, and returns a `Rectangle` with its lower left corner on the point, and a width and height corresponding to the `f32`.

### See also:

[`attributes`][attributes] and [destructuring][destructuring]

[attributes]: attribute.html
[c_struct]: https://en.wikipedia.org/wiki/Struct_(C_programming_language)
[destructuring]: flow_control/match/destructuring.html

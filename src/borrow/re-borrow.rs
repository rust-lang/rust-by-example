struct Point {
    x: int,
    y: int,
    z: int,
}

fn main() {
    let mut point = Point { x: 0, y: 0, z: 0 };

    if true {
        let borrowed_point = &point;
        let another_borrow = &point;

        // data can be accessed via the references and the original owner
        println!("Point has coordinates: ({}, {}, {})",
                 borrowed_point.x,
                 another_borrow.y,
                 point.z);

        // Error: cannot borrow `point` as mutable because it is also
        // borrowed as immutable
        //let mutable_borrow = &mut point;

        // immutables references go out of scope
    }

    if true {
        let mutable_borrow = &mut point;

        // change data via mutable reference
        mutable_borrow.x = 5;

        // copies of the mutably borrowed struct are allowed
        let copied_point = point;

        // Error: cannot borrow `point` as immutable because it is also
        // borrowed as mutable
        //let y = &point.y;

        // Error: can't print, because println! takes an immutable
        // reference
        //println!("Point Z coordinate is {}", point.z);

        // mutable reference goes out of scope
    }

    // immutable references to point are allowed again
    println!("Point now has coordinates: ({}, {}, {})",
             point.x, point.y, point.z);
}

struct Point {
    x: int,
    y: int,
}

fn main() {
    let point = Point { x: 0, y: 0 };

    // superfluous type annotation
    let copy_of_x: int = {
        // ref_to_x is a reference to the x field of point
        // ref_to_x type signature is &int
        let Point { x: ref ref_to_x, y: _ } = point;

        // return a copy of the x field of point
        *ref_to_x
    };

    // a mutable copy of point
    let mut mutable_point = point;

    if true {
        // ref can be paired with mut to take mutable references
        let Point { x: _, y: ref mut mut_ref_to_y } = mutable_point;

        // mutate y field of mutable_point, via a mutable reference
        *mut_ref_to_y = 1;
    }

    println!("point is ({}, {})", point.x, point.y);
    println!("mutable_point is ({}, {})",
             mutable_point.x, mutable_point.y);
}

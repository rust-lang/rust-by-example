struct Point { x: int, y: int }

fn main() {
    let point = Point { x: 0, y: 0 };

    let _copy_of_x = {
        // `ref_to_x` is a reference to the `x` field of `point`
        let Point { x: ref ref_to_x, y: _ } = point;

        // Return a copy of the `x` field of `point`
        *ref_to_x
    };

    // A mutable copy of `point`
    let mut mutable_point = point;

    {
        // `ref` can be paired with `mut` to take mutable references
        let Point { x: _, y: ref mut mut_ref_to_y } = mutable_point;

        // Mutate the `y` field of `mutable_point`, via a mutable reference
        *mut_ref_to_y = 1;
    }

    println!("point is ({}, {})", point.x, point.y);
    println!("mutable_point is ({}, {})", mutable_point.x, mutable_point.y);

    let mut tuple = (box 5u, 3u);

    {
        // `ref` can also be paired with `box` to take a mutable reference to
        // the data contained in the box
        let (box ref mut i, _) = tuple;

        *i = 3;
    }

    println!("tuple is {}", tuple);
}

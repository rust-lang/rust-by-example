use std::mem::size_of_val;

// This function borrows a slice
fn analyze_slice(slice: &[int]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // fixed-size array (type signature is superfluous)
    let xs: [int, ..5] = [1, 2, 3, 4, 5];

    // indexing starts at 0
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // len() returns the size of the array
    println!("array size: {}", xs.len());

    // arrays are stack allocated
    println!("array occupies {} bytes", size_of_val(&xs));

    // out of bound indexing yields a runtime failure
    //println!("{}", xs[5]);

    // arrays can be automatically converted to slices
    println!("borrow the whole array as an slice");
    analyze_slice(xs);

    // slices can point to a section of an array
    println!("borrow a section of the array as an slice");
    analyze_slice(xs.slice(1, 4));
}

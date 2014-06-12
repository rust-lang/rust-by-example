use std::mem;

// This function borrows a slice
fn analyze_slice(slice: &[int]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // Fixed-size array (type signature is superfluous)
    let xs: [int, ..5] = [1, 2, 3, 4, 5];

    // Indexing starts at 0
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len` returns the size of the array
    println!("array size: {}", xs.len());

    // Arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices
    println!("borrow the whole array as a slice");
    analyze_slice(xs);

    // Slices can point to a section of an array
    println!("borrow a section of the array as a slice");
    analyze_slice(xs.slice(1, 4));

    // Out of bound indexing yields a task failure
    println!("{}", xs[5]);
}

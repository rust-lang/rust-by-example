fn main() {
    let immutable_box = box 5;

    println!("immutable_box contains {}", immutable_box);

    // Mutability error
    //*immutable_box = 4;

    // hand over box, change mutability
    let mut mutable_box = immutable_box;

    println!("mutable_box contains {}", mutable_box);

    // modify contents of the box
    *mutable_box = 4;

    println!("mutable_box now contains {}", mutable_box);
}

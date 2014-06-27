fn main() {
    let immutable_box = box 5u;

    println!("immutable_box contains {}", immutable_box);

    // Mutability error
    //*immutable_box = 4;

    // Hand over the box, changing the mutability
    let mut mutable_box = immutable_box;

    println!("mutable_box contained {}", mutable_box);

    // Modify the contents of the box
    *mutable_box = 4;

    println!("mutable_box now contains {}", mutable_box);
}

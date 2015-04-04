fn main() {
    let immutable_box = Box::new(5u32);

    println!("immutable_box contains {}", immutable_box);

    // Mutability error
    //*immutable_box = 4;

    // Hand over the box, changing the mutability
    let mut mutable_box = immutable_box;

    println!("mutable_box contained {}", mutable_box);

    // Modify the contents of the box
    *mutable_box = 4;

    println!("mutable_box now contains {}", mutable_box);

    // on the other hand, suppose the variable itself is immutable
    let x = 5u32;
    let immutable_ref = &x;

    println!("immutable_ref contains {}", immutable_ref);

    // Mutability error
    //*immutable_ref = 4;

    // This is allowed, since the compiler can detect that "variable does not
    // need to be mutable", but generates a warning that should be fixed.
    let mut mutable_ref = immutable_ref;

    println!("mutable_ref to immutable value still contains {}", mutable_ref);

    // However, "const correctness" is enforced, and the following causes a
    // mutability error at compile time!
    //*mutable_ref += 1;

    // the underlying value must be mutable for a mutable reference to exist
    let mut x = 5u32;
    let ref mut mutable_ref = x;
    println!("mutable_ref to mutable value contains {}", mutable_ref);

    // so this works
    *mutable_ref = 4;
    println!("mutable_ref to mutable value contains {}", mutable_ref);
}

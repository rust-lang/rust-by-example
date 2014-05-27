fn main() {
    // By local inference, the compiler knows that the type of `e` is uint
    let e = 5u;

    // Create an empty vector (a growable array)
    let mut v = Vec::new();
    // At this point the compiler doesn't know the exact type of `v`, it
    // just knows that it's a vector of something

    // Insert `e` in the vector
    v.push(e);
    // Aha! Now the compiler knows that `v` is a vector of uints.
}

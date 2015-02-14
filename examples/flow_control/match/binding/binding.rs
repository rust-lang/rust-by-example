fn main() {
    let age = 3;
    // TODO ^ Try different values for `age`

    println!("Tell me type of person you are");
    match age {
        // Bind to `n` for the sequence of 1 through 12.
        n @ 1  ... 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ... 19 => println!("I'm a teen of age {:?}", n),
        // Nothing bound. Return the result.
        n             => println!("I'm an old person of age {:?}", n),
    }
}

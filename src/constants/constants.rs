static LANGUAGE: &'static str = "Rust";
static THRESHOLD: int = 10;

fn is_big(n: int) -> bool {
    // access constant in some function
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // access constant in the main loop
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Error: cannot modify static item
    //THRESHOLD = 5;

    if true {
        // string literals are placed in read-only memory
        let static_string: &'static str = "In read-only memory";

        // when static_string goes out of scope, we can't no longer refer
        // to it, but the string remains in the read-only memory
    }
}

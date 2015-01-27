static LANGUAGE: &'static str = "Rust";
static THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // Access constant in the main task
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Error! Cannot modify a static item
    THRESHOLD = 5;
    // FIXME ^ Comment out this line

    {
        // String literals are references to read-only memory
        let _static_string: &'static str = "In read-only memory";

        // When `_static_string` goes out of scope, we can no longer refer to
        // the underlying data, but the string remains in the read-only memory
    }
}

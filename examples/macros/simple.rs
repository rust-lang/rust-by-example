// This is the simplest macro, `say_hello` is the name of the macro
macro_rules! say_hello {
    // `()` indicates that the macro takes no argument
    () => (
        // the macro will expand into the contents of this block
        println!("Hello!");
    )
}

fn main() {
    // this call will expand into `println!("Hello");`
    say_hello!()
}

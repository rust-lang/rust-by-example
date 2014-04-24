fn main() {
    let captured_value = 7;

    let closure = |argument| {
        println!("I captured this: {}", captured_value);
        println!("argument passed was: {}", argument);
        true
    };

    println!("closure returned: {}", closure("a string"));
}

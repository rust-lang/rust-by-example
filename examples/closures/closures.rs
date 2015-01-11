fn main() {
    let captured_value = 7us;

    let closure = |&:argument| {
        println!("I captured this: {}", captured_value);
        println!("Argument passed was: {}", argument);

        true
    };

    println!("Closure returned: {}", closure("a string"));
}

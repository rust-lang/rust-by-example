// Define a function which takes a function as an argument and calls it.
fn call_function<F: Fn()>(f: F) {
    f()
}

// Define a simple function to be used as an input.
fn print() {
    println!("I'm a function!")
}

fn main() {
    // Define a closure similar to the `print()` function above.
    let closure = || println!("I'm a closure!");
    
    call_function(closure);
    call_function(print);
}
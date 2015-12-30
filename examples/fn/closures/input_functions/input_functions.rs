// A function which takes a closure as an argument and calls it.
fn call_function<F: Fn()>(f: F) {
    f()
}

fn print() {
    println!("I'm a function! I can be used like a closure.")
}

fn main() {
    let closure = || println!("I'm a closure!");
    call_function(closure);
    
    call_function(print);
}
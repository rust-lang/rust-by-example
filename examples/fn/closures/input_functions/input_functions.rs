fn call_function<F: Fn()>(f: F) {
    f()
}

fn print() { println!("I'm a function!") }

fn main() {
    call_function(print);
}

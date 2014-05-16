// this crate is a library
#![crate_type = "lib"]
// the library is named "erty", and its version is 0.1
#![crate_id = "erty#0.1"]

pub fn public_function() {
    println!("called erty's public_function()");
}

fn private_function() {
    println!("called erty's private_function()");
}

pub fn indirect_access() {
    print!("called erty's indirect_access(), that\n> ");

    private_function();
}

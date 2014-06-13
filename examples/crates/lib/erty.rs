pub fn public_function() {
    println!("called erty's `public_function()`");
}

fn private_function() {
    println!("called erty's `private_function()`");
}

pub fn indirect_access() {
    print!("called erty's `indirect_access()`, that\n> ");

    private_function();
}

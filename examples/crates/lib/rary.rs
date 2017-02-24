pub fn public_function() {
    println!("вызвана `public_function()` контейнера rary");
}

fn private_function() {
    println!("вызвана `private_function()` контейнера rary");
}

pub fn indirect_access() {
    print!("вызвана `indirect_access()` контейнера rary, которая\n> ");

    private_function();
}

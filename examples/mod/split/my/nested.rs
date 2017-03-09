pub fn function() {
    println!("вызвана `my::nested::function()`");
}

#[allow(dead_code)]
fn private_function() {
    println!("вызвана `my::nested::private_function()`");
}

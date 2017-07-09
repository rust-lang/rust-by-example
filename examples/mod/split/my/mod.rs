// Точно так же, `mod inaccessible` и `mod nested` обнаружат файлы `nested.rs`
// и `inaccessible.rs`, и затем вставят их здесь в соответствующие модули

mod inaccessible;
pub mod nested;

pub fn function() {
    println!("вызвана `my::function()`");
}

fn private_function() {
    println!("вызывает `my::private_function()`");
}

pub fn indirect_access() {
    print!("вызвана `my::indirect_access()`, которая\n> ");

    private_function();
}

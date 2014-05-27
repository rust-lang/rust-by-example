fn function() {
    println!("called function()");
}

// a module named `my`
mod my {
    // a module can contain items like functions
    fn function() {
        println!("called my::function()");
    }

    // modules can be nested
    mod nested {
        fn function() {
            println!("called my::nested::function()");
        }
    }
}

fn main() {
    function();

    // items inside a module can be called using the full path
    my::function();
}

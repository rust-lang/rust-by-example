fn function() {
    println!("called `function()`");
}

// A module named `my`
mod my {
    // A module can contain items like functions
    #[allow(dead_code)]
    fn function() {
        println!("called `my::function()`");
    }

    // Modules can be nested
    mod nested {
        #[allow(dead_code)]
        fn function() {
            println!("called `my::nested::function()`");
        }
    }
}

fn main() {
    function();

    // Error! `my::function` is private
    my::function();
    // TODO ^ Comment out this line
}

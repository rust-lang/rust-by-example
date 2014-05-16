fn function() {
    println!("called function()");
}

mod my {
    // a public function
    pub fn function() {
        println!("called my::function()");
    }

    // a private function
    fn private_function() {
        println!("called my::private_function()");
    }

    // items can access other items in the same module
    pub fn indirect_access() {
        print!("called my::indirect_access(), that\n> ");

        // regardless of their visibility
        private_function();
    }

    // a public module
    pub mod nested {
        pub fn function() {
            println!("called my::nested::function()");
        }

        fn private_function() {
            println!("called my::nested::private_function()");
        }
    }

    // a private module
    mod inaccessible {
        pub fn public_function() {
            println!("called my::inaccessible::public_function()");
        }
    }
}

fn main() {
    // the public items of a module can be accessed
    my::function();

    // modules allow disambiguation between items that have the same name
    function();

    // the private items of a module can't be directly accessed
    // Error: private_function is private
    //my::private_function();

    my::indirect_access();

    // public items inside public nested modules can be accessed from outside
    // the parent module
    my::nested::function();

    // but private items inside public nested modules can't be accessed
    // Error: private_function is private
    //my::nested::private_function();

    // items inside private nested modules can't be accessed, regardless of
    // their visibility
    // Error: inaccessible is a private module
    //my::inaccessible::public_function();
}

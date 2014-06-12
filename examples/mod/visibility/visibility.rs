fn function() {
    println!("called `function()`");
}

mod my {
    // A public function
    pub fn function() {
        println!("called `my::function()`");
    }

    // A private function
    fn private_function() {
        println!("called `my::private_function()`");
    }

    // Items can access other items in the same module
    pub fn indirect_access() {
        print!("called `my::indirect_access()`, that\n> ");

        // regardless of their visibility
        private_function();
    }

    // A public module
    pub mod nested {
        pub fn function() {
            println!("called `my::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my::nested::private_function()`");
        }
    }

    // A private module
    mod inaccessible {
        #[allow(dead_code)]
        pub fn public_function() {
            println!("called `my::inaccessible::public_function()`");
        }
    }
}

fn main() {
    // The public items of a module can be accessed
    my::function();

    // modules allow disambiguation between items that have the same name
    function();

    // The private items of a module can't be directly accessed
    // Error! `private_function` is private
    //my::private_function();
    // TODO ^ Try uncommenting this line

    my::indirect_access();

    // Public items inside public nested modules can be accessed from outside
    // the parent module
    my::nested::function();

    // but private items inside public nested modules can't be accessed
    // Error! `private_function` is private
    //my::nested::private_function();
    // TODO ^ Try uncommenting this line

    // Items inside private nested modules can't be accessed, regardless of
    // their visibility
    // Error! `inaccessible` is a private module
    //my::inaccessible::public_function();
    // TODO ^ Try uncommenting this line
}

fn function() {
    println!("called function()");
}

mod my {
    pub fn indirect_call() {
        // let's see how to access all the functions named `function` from this
        // scope
        print!("called my::indirect_call(), that\n> ");

        // my::function() can be called directly
        function();

        if true {
            // this will bind to the cool::function in the *root* scope
            // (it's actually the crate scope, but we haven't talk about crates
            // yet)
            use root_cool_function = cool::function;

            print!("> ");
            root_cool_function();
        }

        if true {
            // `self` refers to the current module scope, in this case: `my`
            use my_cool_function = self::cool::function;

            print!("> ")
            my_cool_function();
        }

        if true {
            // `super` refers to the parent scope, i.e. outside of the `my`
            // module
            use root_function = super::function;

            print!("> ");
            root_function();
        }
    }

    fn function() {
        println!("called my::function()");
    }

    mod cool {
        pub fn function() {
            println!("called my::cool::function()");
        }
    }
}

mod cool {
    pub fn function() {
        println!("called cool::function()");
    }
}

fn main() {
    my::indirect_call();
}

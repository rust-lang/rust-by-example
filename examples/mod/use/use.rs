// Bind the `deeply::nested::function` path to `other_function`
use other_function = deeply::nested::function;

fn function() {
    println!("called `function()`");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`")
        }
    }
}

fn main() {
    // Easier access to `deeply::nested::function`
    other_function();

    println!("Entering block");
    {
        // This is equivalent to `use function = deeply::nested::function`
        // This `function` will shadow the outer one
        use deeply::nested::function;

        function();

        println!("Leaving block");

        // `use` bindings have a local scope, in this case the `function`
        // shadowing is only available in this scope
    }

    function();
}

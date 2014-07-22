#![feature(macro_rules)]

macro_rules! create_function {
    // This macro takes an argument of "type" `ident`
    // The `ident` designator is used for variable/function names
    ($func_name:ident) => {
        // This macro creates a function with name `$func_name`
        fn $func_name() {
            // The `stringify!` macro converts an `ident` into a string
            println!("You called {}()", stringify!($func_name))
        }
    }
}

create_function!(foo)
create_function!(bar)
// Designators provide type safety in macros
//create_function!(0)
// TODO ^ Try uncommenting this line

macro_rules! print_result {
    // The `expr` designator is used for expressions
    ($expression:expr) => {
        // `stringify!` will convert the expression *as it is* into a string
        println!("{} = {}", stringify!($expression), $expression)
    }
}

fn main() {
    foo();
    bar();

    print_result!(1u + 1);

    // Remember that blocks are expressions too
    print_result!({
        let x = 1u;

        x * x + 2 * x - 1
    });
}

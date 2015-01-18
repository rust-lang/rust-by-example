macro_rules! create_function {
    // this macro takes an argument of "type" `ident`
    // the `ident` designator is used for variable/function names
    ($func_name:ident) => (
        // this macro creates a function with name `$func_name`
        fn $func_name() {
            // the stringify! macro converts an `ident` into a string
            println!("You called {:?}()",
                     stringify!($func_name))
        }
    )
}

create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    // the `expr` designator is used for expressions
    ($expression:expr) => (
        // stringify! will convert the expression *as it is* into a string
        println!("{:?} = {:?}",
                 stringify!($expression),
                 $expression)
    )
}

fn main() {
    foo();
    bar();

    print_result!(1u32 + 1);

    // remember that blocks are expressions
    print_result!({
        let x = 1u32;

        x * x + 2 * x - 1
    });
}

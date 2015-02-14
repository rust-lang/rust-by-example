use std::env;

fn main() {
    let args: Vec<String> = env::args().map(|x| x.to_string())
                                       .collect();
    // The first argument is the path that was used to call the program.
    println!("My path is {}.", args[0]);
    // The rest of the arguments are the passed command line parameters.
    // Call the program like this:
    // $ ./args arg1 arg2
    println!("I got {:?} arguments: {:?}.", args.len() - 1, args.tail());
}

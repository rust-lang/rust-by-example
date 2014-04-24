fn main() {
    // print! is like println! but without adding a newline at the end
    print!("January has ");

    // {} are placeholders for arguments that will be stringified
    println!("{} days", 31);

    // the positional arguments can be reused along the template
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // named arguments can also be used
    println!("{subject} {verb} {predicate}",
             predicate="over the lazy dog",
             subject="the quick brown fox",
             verb="jumps");

    // special formatting can be specified in the placeholder after a `:`
    println!("{} of {:t} people know binary, the other half don't", 1, 2);

    // Error: you are missing an argument
    //println!("My name is {0}, {1} {0}", "Bond");
}

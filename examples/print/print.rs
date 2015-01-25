fn main() {
    // `print!` is like `println!` but it doesn't add a newline at the end
    print!("January has ");

    // `{}` are placeholders for arguments that will be stringified
    println!("{} days", 31);
    // The `i` suffix indicates the compiler that this literal has type: signed
    // pointer size integer, see next chapter for more details

    // The positional arguments can be reused along the template
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // Named arguments can also be used
    println!("{subject} {verb} {predicate}",
             predicate="over the lazy dog",
             subject="the quick brown fox",
             verb="jumps");

    // Special formatting can be specified in the placeholder after a `:`
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // Error! You are missing an argument
    println!("My name is {0}, {1} {0}", "Bond");
    // FIXME ^ Add the missing argument: "James"
}

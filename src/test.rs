mod escape {
    use playpen;

    #[test]
    fn hello() {
        let inp = playpen::escape("// the main function
fn main() {
    // print to the console
    println!(\"Hello World!\");
}");

        let out = "%2F%2F%20the%20main%20function%0Afn%20main()%20%7B%0A%20%20%20%20%2F%2F%20print%20to%20the%20console%0A%20%20%20%20println!(%22Hello%20World!%22)%3B%0A%7D";

        assert_eq!(format!("\n{}\n", inp), format!("\n{}\n", out));
    }

    #[test]
    fn print() {
        let inp = playpen::escape("fn main() {
    // print! is like println! but without adding a newline at the end
    print!(\"January has \");

    // {} are placeholders for arguments that will be stringified
    println!(\"{} days\", 31);

    // the positional arguments can be reused along the template
    println!(\"{0}, this is {1}. {1}, this is {0}\", \"Alice\", \"Bob\");

    // named arguments can also be used
    println!(\"{subject} {verb} {predicate}\",
             predicate=\"over the lazy dog\",
             subject=\"the quick brown fox\",
             verb=\"jumps\");

    // special formatting can be specified in the placeholder after a `:`
    println!(\"{} of {:t} people know binary, the other half don't\", 1, 2);

    // Error: you are missing an argument
    //println!(\"My name is {0}, {1} {0}\", \"Bond\");
}");

        let out = "fn%20main()%20%7B%0A%20%20%20%20%2F%2F%20print!%20is%20like%20println!%20but%20without%20adding%20a%20newline%20at%20the%20end%0A%20%20%20%20print!(%22January%20has%20%22)%3B%0A%0A%20%20%20%20%2F%2F%20%7B%7D%20are%20placeholders%20for%20arguments%20that%20will%20be%20stringified%0A%20%20%20%20println!(%22%7B%7D%20days%22%2C%2031)%3B%0A%0A%20%20%20%20%2F%2F%20the%20positional%20arguments%20can%20be%20reused%20along%20the%20template%0A%20%20%20%20println!(%22%7B0%7D%2C%20this%20is%20%7B1%7D.%20%7B1%7D%2C%20this%20is%20%7B0%7D%22%2C%20%22Alice%22%2C%20%22Bob%22)%3B%0A%0A%20%20%20%20%2F%2F%20named%20arguments%20can%20also%20be%20used%0A%20%20%20%20println!(%22%7Bsubject%7D%20%7Bverb%7D%20%7Bpredicate%7D%22%2C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20predicate%3D%22over%20the%20lazy%20dog%22%2C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20subject%3D%22the%20quick%20brown%20fox%22%2C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20verb%3D%22jumps%22)%3B%0A%0A%20%20%20%20%2F%2F%20special%20formatting%20can%20be%20specified%20in%20the%20placeholder%20after%20a%20%60%3A%60%0A%20%20%20%20println!(%22%7B%7D%20of%20%7B%3At%7D%20people%20know%20binary%2C%20the%20other%20half%20don%27t%22%2C%201%2C%202)%3B%0A%0A%20%20%20%20%2F%2F%20Error%3A%20you%20are%20missing%20an%20argument%0A%20%20%20%20%2F%2Fprintln!(%22My%20name%20is%20%7B0%7D%2C%20%7B1%7D%20%7B0%7D%22%2C%20%22Bond%22)%3B%0A%7D";

        assert_eq!(format!("\n{}\n", inp), format!("\n{}\n", out));
    }
}


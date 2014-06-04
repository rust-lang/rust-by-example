fn main() {
    // (all the type annotations are superfluous)
    // a reference to a string allocated in read only memory
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("pangram: {}", pangram);

    // iterate over words in reverse, no new string is allocated
    println!("words in reverse");
    for word in pangram.words().rev() {
        println!("{}", word);
    }

    // copy chars into a vector, sort and remove duplicates
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();

    // a growable `String`
    let mut string: String = String::new();
    for c in chars.move_iter() {
        // insert a char at the end of string
        string.push_char(c);
        // insert a string at the end of string
        string.push_str(", ");
    }

    // the trimmed string is a slice to the original string, hence
    // no new allocation is performed
    let trimmed_str: &str = string.as_slice().trim_chars(&[',', ' ']);
    println!("used characters: {}", trimmed_str);

    // heap allocate a string
    let alice = String::from_str("I like dogs");
    // allocate new memory and store the modified string there
    let bob: String = alice.replace("dog", "cat");

    println!("Alice says: {}", alice);
    println!("Bob says: {}", bob);
}

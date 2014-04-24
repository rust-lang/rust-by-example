fn main() {
    let pangram = "the quick brow fox jumps over the lazy dog";
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

    // StrBuf is a growable array
    let mut strbuf = StrBuf::new();
    for c in chars.move_iter() {
        // insert a char at the end of strbuf
        strbuf.push_char(c);
        // insert a string at the end of strbuf
        strbuf.push_str(", ");
    }

    // the trimmed string is a slice to the original string, hence
    // no new allocation is performed
    let trimmed_string = strbuf.as_slice().trim_chars(&[',', ' ']);
    println!("used characters: {}", trimmed_string);

    // heap allocate a string
    let alice = ~"I like dogs";
    // replaced string gets heap allocated (superfluous type annotation)
    let bob: ~str = alice.replace("dog", "cat");

    println!("Bob says: {}", bob);
}

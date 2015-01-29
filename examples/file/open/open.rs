use std::old_io::File;

fn main() {
    // Create a path to the desired file
    let path = Path::new("hello.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `IoResult<File>`
    let mut file = match File::open(&path) {
        // The `desc` field of `IoError` is a string that describes the error
        Err(why) => panic!("couldn't open {}: {}", display, why.desc),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `IoResult<String>`
    match file.read_to_string() {
        Err(why) => panic!("couldn't read {}: {}", display, why.desc),
        Ok(string) => print!("{} contains:\n{}", display, string),
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed
}

use std::io::File;

fn main() {
    // create a path
    let path = Path::new("hello.txt");

    // open the path in read-only mode, returns IoResult<File>
    let mut file = match File::open(&path) {
        // the desc field of IoError is a string that describes the problem
        Err(why) => fail!("couldn't open {}: {}", path.display(), why.desc),
        Ok(file) => file,
    };

    // read the file contents into a string, returns IoResult<String>
    match file.read_to_str() {
        Err(why) => fail!("couldn't read {}: {}", path.display(), why.desc),
        Ok(string) => print!("{} contains:\n{}", path.display(), string),
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed
}

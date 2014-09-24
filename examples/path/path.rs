use std::io::fs::PathExtensions;

fn main() {
    // Create a `Path` from an `&'static str`
    let path = Path::new(".");

    // The `display` method returns a `Show`able structure
    let display = path.display();

    // Check if the path exists
    if path.exists() {
        println!("{} exists", display);
    }

    // Check if the path is a file
    if path.is_file() {
        println!("{} is a file", display);
    }

    // Check if the path is a directory
    if path.is_dir() {
        println!("{} is a directory", display);
    }

    // `stat` returns an IoResult<FileStat> === Result<FileStat, IoError>
    let stat = match path.stat() {
        Err(why) => fail!("{}", why.desc),
        Ok(stat) => stat,
    };

    println!("{} size is {} bytes", display, stat.size);

    // `join` merges a path with a byte container using the OS specific
    // separator, and returns the new path
    let new_path = path.join("a").join("b");

    // Convert the path into a string slice
    match new_path.as_str() {
        None => fail!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }
}

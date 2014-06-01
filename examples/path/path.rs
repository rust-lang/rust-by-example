fn main() {
    // create a Path from an &'static str
    let path = Path::new("path.rs");

    // check if the path exists
    if path.exists() {
        // the `display` method makes the path showable
        println!("{} exists", path.display());
    }

    // check if the path is a file
    if path.is_file() {
        println!("{} is a file", path.display());
    }

    // check if the path is a dir
    if path.is_dir() {
        println!("{} is a directory", path.display());
    }

    // `stat` returns an IoResult<FileStat> === Result<FileStat, IoError>
    let stat = match path.stat() {
        Err(why) => fail!("{}", why.desc),
        Ok(stat) => stat,
    };

    println!("{} size is {} bytes", path.display(), stat.size);

    let dir = Path::new("a");

    // `join` merges a path with a byte container using the OS specific
    // separator
    let new_path = dir.join(path);

    match new_path.as_str() {
        None => fail!("new path is not a valid UTF-8 sequence"),
        Some(string_slice) => println!("new path is {}", string_slice),
    }
}

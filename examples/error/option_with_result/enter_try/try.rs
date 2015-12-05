use std::io::prelude::*;
use std::fs::File;

type Result<T> = std::result::Result<T, String>;

// Setup to make this work. Create two files with some info.
fn setup() -> std::io::Result<()> {
    let mut a = try!(File::create("a"));
    try!(a.write_all(b"grape"));

    let mut b = try!(File::create("b"));
    b.write_all(b"fruit")
}

// Get the data from each file with the data stored in a `Result`.
fn get_data(path: &str) -> Result<String> {
    // `try` unwraps the value or returns the error.
    let mut file = try!(File::open(path)
        // Errors still must be converted to strings.
        .map_err(|err| err.to_string())
    );
    let mut contents = String::new();

    // Read the data into `contents`.
    try!(file.read_to_string(&mut contents)
        .map_err(|err| err.to_string())
    );

    Ok(contents)
}

// Concat the contents of the two files together into a new `Result`.
fn concat(a: &str, b: &str) -> Result<String> {
    let (data_a, data_b) = (try!(get_data(a)), try!(get_data(b)));

    Ok(data_a + &data_b)
}

fn main() {
    // Ignore this result.
    setup().unwrap();

    match concat("a", "b") {
        Ok(n)  => println!("{}", n),
        Err(e) => println!("Error: {}", e),
    }
}

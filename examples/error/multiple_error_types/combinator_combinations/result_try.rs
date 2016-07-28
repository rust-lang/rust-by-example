use std::io::prelude::*;
use std::fs::File;

type Result<T> = std::result::Result<T, String>;

// Setup to make this work. Create two files with some info. Ignore the
// return values because we don't care about them here.
fn setup() {
    File::create("a")
        .and_then(|mut file| file.write_all(b"grape"))
        .unwrap();

    File::create("b")
        .and_then(|mut file| file.write_all(b"fruit"))
        .unwrap();
}

// Get the data from each file with the data stored in a `Result`.
fn get_data(path: &str) -> Result<String> {
    File::open(path)
        .map_err(|err| err.to_string())
        .and_then(|mut file| {
            let mut contents = String::new();

            // Read the data into `contents`.
            file.read_to_string(&mut contents)
                .map_err(|err| err.to_string())
                // Ignore the output `read_to_string` returns and return `contents`.
                .map(|_| contents)
        })
}

// Concat the contents of the two files together into a new `Result`.
fn concat(filename_a: &str, filename_b: &str) -> Result<String> {
    let (data_a, data_b) = (get_data(filename_a), get_data(filename_b));
    
    data_a.and_then(|a|
        // Return `Ok` when both `a` and `b` are `Ok`. Otherwise return
        // whichever has the first `Err`.
        data_b.and_then(|b| Ok(a + &b))
    )
}

fn main() {
    setup();

    match concat("a", "b") {
        Ok(n)  => println!("{}", n),
        Err(e) => println!("Error: {}", e),
    }
}

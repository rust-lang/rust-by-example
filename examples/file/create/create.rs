use std::io::File;

static LOREM_IPSUM: &'static str =
"Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

fn main() {
    let path = Path::new("out/lorem_ipsum.txt");
    let display = path.display();

    // Open a file in write-only mode, returns `IoResult<File>`
    let mut file = match File::create(&path) {
        Err(why) => fail!("couldn't create {}: {}", display, why.desc),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `IoResult<()>`
    match file.write_str(LOREM_IPSUM) {
        Err(why) => {
            fail!("couldn't write to {}: {}", display, why.desc)
        },
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

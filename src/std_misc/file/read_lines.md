# `read_lines`

## Beginner friendly method

This method is NOT efficient. It's here for beginners
who can't understand the efficient method yet.

```rust,no_run
use std::fs::File;
use std::io::Read;

fn read_lines(filename: String) -> Vec<String> {
    // Open the file in read-only mode.
    let mut file = File::open(filename).unwrap();
    // Read file contents into a String buffer.
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    // Parse the buffer line by line, and collect into a `Vec<String>`
    buffer.lines().map(|line| line.to_owned()).collect()
}

fn main() {
    // Stores a Vector of lines of the file in lines variable.
    let lines = read_lines("./hosts".to_string());
    // Iterate over the lines of the file, and in this case print them.
    for line in lines {
        println!("{}", line);
    }
}
```

Running this program simply prints the lines individually.

```shell
$ echo -e "127.0.0.1\n192.168.0.1\n" > hosts
$ rustc read_lines.rs && ./read_lines
127.0.0.1
192.168.0.1
```

## Efficient method

The method `lines()` returns an iterator over the lines
of a file.

`File::open` expects a generic, `AsRef<Path>`. That's what
`read_lines()` expects as input.

```rust,no_run
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./hosts") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
```

Running this program simply prints the lines individually.

```shell
$ echo -e "127.0.0.1\n192.168.0.1\n" > hosts
$ rustc read_lines.rs && ./read_lines
127.0.0.1
192.168.0.1
```

This process is more efficient than creating a `String` in memory
especially working with larger files.

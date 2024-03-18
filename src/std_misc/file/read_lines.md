# `read_lines`

## A naive approach

This might be a reasonable first attempt for a beginner's first implementation
for reading lines from a file.

```rust,norun
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
```

Since the method `lines()` returns an iterator over the lines in the file, we
can also perform a map inline and collect the results, yielding a more concise
and fluent expression.

```rust,norun
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}
```

Note that in both examples above, we must convert the `&str` reference returned
from `lines()` to the owned type `String`, using `.to_string()` and
`String::from` respectively.

## A more efficient approach

Here we pass ownership of the open `File` to a `BufReader` struct. `BufReader`
uses an internal buffer to reduce intermediate allocations.

We also update `read_lines` to return an iterator instead of allocating new
`String` objects in memory for each line.

```rust,no_run
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./hosts.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            println!("{}", line);
        }
    }
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
```

Running this program simply prints the lines individually.

```shell
$ echo -e "127.0.0.1\n192.168.0.1\n" > hosts.txt
$ rustc read_lines.rs && ./read_lines
127.0.0.1
192.168.0.1
```

(Note that since `File::open` expects a generic `AsRef<Path>` as argument, we
define our generic `read_lines()` method with the same generic constraint, using
the `where` keyword.)

This process is more efficient than creating a `String` in memory with all of
the file's contents. This can especially cause performance issues when working
with larger files.

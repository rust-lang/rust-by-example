#![feature(io)]

use std::process::Command;
use std::io::prelude::*;

static PANGRAM: &'static str =
"the quick brown fox jumped over the lazy dog\n";

fn main() {
    // Spawn the `wc` command
    let process = match Command::new("wc").spawn() {
        Err(why) => panic!("couldn't spawn wc: {}", why.description()),
        Ok(process) => process,
    };

    {
        // Write a string to the stdin of `wc`
        match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
            Err(why) => panic!("couldn't write to wc stdin: {}",
                               why.description()),
            Ok(_) => println!("sent pangram to wc"),
        }

        // `stdin` gets `drop`ed here, and the pipe is closed
        // This is very important, otherwise `wc` wouldn't start processing the
        // input we just sent
    }

    // The `stdout` field also has type `Option<PipeStream>`
    // the `as_mut` method will return a mutable reference to the value
    // wrapped in a `Some` variant
    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read wc stdout: {}",
                           why.description()),
        Ok(_) => print!("wc responded with:\n{}", s),
    }
}

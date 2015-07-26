#![feature(rustc_private)]

extern crate getopts;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let opts = [
        getopts::optflag("a", "long_a", ""),
        getopts::optflag("b", "long_b", ""),
        getopts::optopt("c", "long_c", "", "VALUE"),
        //^ Use `optflagopt` if the argument should be optional.
        //  Use `reqopt` if the option is required.
        //  Use `optmulti`, `optflagmulti` if options can occur multiple times.
    ];

    let matches = match getopts::getopts(&args[1..], &opts) {
        Ok(m) => m,
        Err(f) => {
            println!("{}", f);
            process::exit(1);
        }
    };
    let a = if matches.opt_present("a") {true} else {false};
    let b = if matches.opt_present("b") {true} else {false};
    let c = match matches.opt_str("c") {
        Some(s) => s,
        None => String::from(""),
    };
    //^ Use `matches.opt_default` if you need a default (`opflagopt`).
    //  Use `matches.opt_count` if you need to count how many were matched
    //  (`*multi`).

    println!("a={}, b={}, c=\"{}\"", a, b, c);
    if !matches.free.is_empty() {
        println!("free arguments: {:?}", matches.free);
    }
}

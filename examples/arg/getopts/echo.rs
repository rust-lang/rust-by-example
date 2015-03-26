#![feature(rustc_private)]
#![feature(collections)]
#![feature(exit_status)]

extern crate getopts;

use std::env;
use std::io;
use std::io::Write;

static VERSION: &'static str = "1.0.0";

fn main() {
    let args: Vec<String> = env::args().collect();
    let ref program = args[0];

    // Set possible flags.
    // The first argument to `optflag` is the short flag name.
    // The second argument is the long flag name.
    // The third argument is the help text.
    let opts = [
        getopts::optflag("n", "", "do not output the trailing newline"),
        getopts::optflag("h", "help", "display this help and exit"),
        getopts::optflag("V", "version",
                         "output version information and exit"),
    ];

    let matches = match getopts::getopts(args.tail(), &opts) {
        Ok(m) => m,
        Err(f) => {
            println!("{}", f);
            env::set_exit_status(1);
            return;
            // The exit code is 0 (success) by default.
            // Any exit code other than 0 indicates failure.
        }
    };

    if matches.opt_present("help") {
        //^ We could as well have used the short name: "h"
        println!("echo {} - display a line of text", VERSION);
        println!("");
        println!("Usage:");
        println!(" {} [SHORT-OPTION]... [STRING]...", program);
        println!(" {} LONG-OPTION", program);
        println!("");
        let usage = getopts::usage("Echo the STRING(s) to standard output.",
                                    &opts);
        println!("{}", usage);
        return;
    }

    if matches.opt_present("version") {
        println!("echo version: {}", VERSION);
        return;
    }

    if !matches.free.is_empty() {
        //^ `matches.free` contains all the arguments that are not options.
        let string = matches.free.connect(" ");
        println!("{}", string);
    }

    if !matches.opt_present("n") {
        println!("")
    } else {
        let _ = io::stdout().flush();
    }
}

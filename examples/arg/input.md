The command line arguments can be accessed using `std::os::args`, which returns
a [vector](http://static.rust-lang.org/doc/master/std/vec/index.html) of strings:

{args.rs}

{args.play}

{args.out}

Matching is typically used to parse the arguments:

{match_args.rs}

{match_args.out}

To build unix-style command line interfaces, you can use the [getopts](http://doc.rust-lang.org/getopts/index.html) crate.

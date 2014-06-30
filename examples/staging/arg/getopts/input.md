To build unix-style command line interfaces, you can use the [getopts](http://doc.rust-lang.org/getopts/index.html) crate.

Here is a simple implementation of the `echo` unix program:

{echo.play}

This is a simplified version of the `echo` implementation by [uutils](https://github.com/uutils/coreutils).


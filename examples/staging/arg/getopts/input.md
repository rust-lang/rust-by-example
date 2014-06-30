To build unix-style command line interfaces, you can use the [getopts](http://doc.rust-lang.org/getopts/index.html) crate.

Here is a simple implementation of the `echo` unix program:

{echo.play}

```
$ ./echo -h
echo 1.0.0 - display a line of text

Usage:
 ./echo [SHORT-OPTION]... [STRING]...
 ./echo LONG-OPTION

Echo the STRING(s) to standard output.

Options:
    -n                  do not output the trailing newline
    -h --help           display this help and exit
    -V --version        output version information and exit

$ ./echo --version
echo version: 1.0.0
$ ./echo Hello, World!
Hello, World!
```

This is a simplified version of the `echo` implementation by [uutils](https://github.com/uutils/coreutils).


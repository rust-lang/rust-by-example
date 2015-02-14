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

This is a simplified version of the `echo` implementation by
[uutils](https://github.com/uutils/coreutils).


It is also possible to use *options* instead of *flags*, such that values can
be passed to the program:

{testopt.rs}

Here are some examples how the program behaves given different combinations of
arguments:

```
$ ./testopt
a=false, b=false, c=""
$ ./testopt -a -b
a=true, b=true, c=""
$ ./testopt -ab
a=true, b=true, c=""
$ ./testopt -c
Argument to option 'c' missing.
$ ./testopt -c value
a=false, b=false, c="value"
$ ./testopt -c=value
a=false, b=false, c="=value"
$ ./testopt -cvalue
a=false, b=false, c="value"
$ ./testopt arg
a=false, b=false, c=""
free arguments: [arg]
$ ./testopt -a arg
a=true, b=false, c=""
free arguments: [arg]
$ ./testopt -c value arg
a=false, b=false, c="value"
free arguments: [arg]
$ ./testopt -a -- -b
a=true, b=false, c=""
free arguments: [-b]
$ ./testopt -a -
a=true, b=false, c=""
free arguments: [-]
```

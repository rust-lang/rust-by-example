File I/O in Rust is handled with the `File` struct. Several methods are
associated with `File` for opening, reading or writing a file. Most of these
methods return the `IoResult<T>` type, which is an alias for `Result<T,
IoError>`. This makes the failure of all I/O operations *explicit*, thanks to
this the programmer can see all the failure paths, and is encouraged to handle
them in a proactive manner.

A `File` owns a resource (the file handle), and take cares of closing the file
when the `File` value goes out of scope.

Let's see how to open a file in read-only mode.

{open.rs}

```
$ echo "Hello World!" > hello.txt
$ rustc open.rs && ./open
hello.txt contains:
Hello World!
```

(You are encouraged to test the previous example under failure conditions:
without the hello.txt file, or a hello.txt file without the right permissions,
etc.)

Now let's see how to write to a file.

{create.rs}

```
$ mkdir out
$ rustc create.rs && ./create
successfully wrote to out/lorem_ipsum.txt
$ cat out/lorem_ipsum.txt
Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
```

(As in the previous example, you are encouraged to test this example under
failure conditions)

There is also a more generic
[`open_mode`](http://static.rust-lang.org/doc/master/std/io/fs/struct.File.html#method.open_mode)
function that can open files in read/write mode or append mode.

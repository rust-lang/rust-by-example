The `open` static method can be used to open a file in read-only mode.

A `File` owns a resource, the file descriptor, and take cares of closing the
file when its `drop`ed.

{open.play}

The playpen doesn't allow file I/O, so you'll hit one of the failure paths.
Here's the expected successful output:

```
$ echo "Hello World!" > hello.txt
$ rustc open.rs && ./open
hello.txt contains:
Hello World!
```

(You are encouraged to test the previous example under different failure
conditions: `hello.txt` doesn't exist, or `hello.txt` is not readable,
etc.)

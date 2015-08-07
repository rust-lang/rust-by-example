The `open` static method can be used to open a file in read-only mode.

A `File` owns a resource, the file descriptor and takes care of closing the
file when it is `drop`ed.

{open.rs}

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

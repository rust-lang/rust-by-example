The `create` static method opens a file in write-only mode. If the file already
existed, the old content is destroyed, otherwise a new file is created.

{create.play}

As in the previous example, the playpen won't allow file I/O, so you'll hit one
of the failure paths. Here's the expected successful output:

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
[`open_mode`][open-mode]
method that can open files in other modes like: read+write, append, etc.

[open-mode]: http://doc.rust-lang.org/std/io/fs/struct.File.html#method.open_mode

Rust provides a `Path` struct to deal with file paths in the underlying
filesystem. There are two flavors of `Path`: `posix::Path`, for UNIX-like
systems, and `windows::Path`, for Windows. The prelude exports the appropriate
platform-specific `Path` variant.

A `Path` can be created from almost any type that implements the
`BytesContainer` trait, like a string, and provides several methods to get
information from the file/directory the path points to.

Note that a `Path` is *not* internally represented as an UTF-8 string, but
instead is stored as a vector of bytes (`Vec<u8>`). Therefore, converting a
`Path` to a `&str` is *not* free and may fail (an `Option` is returned).

{path.rs}

<!-- This should be handled by {path.out} :-( -->
```
$ rustc path.rs && ./path
path.rs exists
path.rs is a file
path.rs size is 1074 bytes
new path is a/path.rs
```

Be sure to check at other `Path` methods
([`posix::Path`](http://static.rust-lang.org/doc/master/std/path/posix/struct.Path.html)
or [`windows::Path`](http://static.rust-lang.org/doc/master/std/path/windows/struct.Path.html))
and the
[`FileStat`](http://static.rust-lang.org/doc/master/std/io/struct.FileStat.html)
struct.

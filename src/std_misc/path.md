# Path

The `Path` struct represents file paths in the underlying filesystem. There are
two flavors of `Path`: `posix::Path`, for UNIX-like systems, and
`windows::Path`, for Windows. The prelude exports the appropriate
platform-specific `Path` variant.

A `Path` can be created from an `OsStr`, and provides several methods to get
information from the file/directory the path points to.

A `Path` is immutable. The owned version of `Path` is `PathBuf`. The relation
between `Path` and `PathBuf` is similar to that of `str` and `String`: a
`PathBuf` can be mutated in-place, and can be dereferenced to a `Path`.

Note that a `Path` is *not* internally represented as an UTF-8 string, but
instead is stored as an `OsString`. Therefore, converting a `Path` to a `&str`
is *not* free and may fail (an `Option` is returned). However, a `Path` can be
freely converted to an `OsString` or `&OsStr` using `into_os_string` and
`as_os_str`, respectively.

```rust,editable
use std::path::Path;

fn main() {
    // Create a `Path` from an `&'static str`
    let path = Path::new(".");

    // The `display` method returns a `Display`able structure
    let _display = path.display();

    // `join` merges a path with a byte container using the OS specific
    // separator, and returns a `PathBuf`
    let mut new_path = path.join("a").join("b");

    // `push` extends the `PathBuf` with a `&Path`
    new_path.push("c");
    new_path.push("myfile.tar.gz");

    // `set_file_name` updates the file name of the `PathBuf`
    new_path.set_file_name("package.tgz");

    // Convert the `PathBuf` into a string slice
    match new_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }
}
```

Be sure to check at other `Path` methods (`posix::Path` or `windows::Path`) and
the `Metadata` struct.

### See also:

[OsStr][1] and [Metadata][2].

[1]: https://doc.rust-lang.org/std/ffi/struct.OsStr.html
[2]: https://doc.rust-lang.org/std/fs/struct.Metadata.html

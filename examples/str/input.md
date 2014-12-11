There are two types of strings in Rust: `String` and `&str`.

A `String` is stored as a vector of bytes (`Vec<u8>`), but guaranteed to
always be a valid UTF-8 sequence. `String` is heap allocated, growable and not
null terminated.

`&str` is a slice (`&[u8]`) that always points to a valid UTF-8 sequence, and
can be used to view into a `String`, just like `&[T]` is a view into `Vec<T>`.

{str.play}

More `str`/`String` methods can be found under the
[std::str][str] and
[std::string][string]
modules

[str]: http://doc.rust-lang.org/std/str/
[string]: http://doc.rust-lang.org/std/string/

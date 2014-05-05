Strings in Rust (`str`) are stored as a collection of bytes (`Vec<u8>`), but
are guaranteed to always be valid UTF-8 sequences. `str` are not null
terminated, their size can't be modified, and by extension their content can't
be modified. There are two types of `str`: a heap allocated version `~str`, and
`&str` that can reference a string or a section of a string.

`StrBuf` is a growable string, that provides methods to push characters or
strings into it.

{str.rs}

More `str`/`StrBuf` methods can be found under the
[std::str](http://static.rust-lang.org/doc/master/std/str/index.html) and
[std::strbuf](http://static.rust-lang.org/doc/master/std/strbuf/index.html)
modules

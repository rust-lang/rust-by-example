Strings in Rust are:
* valid UTF-8 sequences
* not null terminated
* always heap allocated

Strings are represented using two words like slices. And `StrBuf` is a growable
string, which internal representation is a `Vec<u8>`.

{str.rs}

More `str`/`StrBuf` methods can be found under the
[std::str](http://static.rust-lang.org/doc/master/std/str/index.html) and
[std::strbuf](http://static.rust-lang.org/doc/master/std/strbuf/index.html)
modules

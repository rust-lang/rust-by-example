# Error handling

Error handling is the process of handling the possibility of failure. For
example, failing to read a file and then continuing to use that *bad* input
would clearly be problematic. Noticing and explicitly managing those errors
saves the rest of the program from various pitfalls.

There are various ways to deal with errors in Rust, which are described in the
following subchapters. They all have more or less subtle differences and
different use cases. As a rule of thumb:

An explicit `panic` is mainly useful for tests and dealing with unrecoverable
errors. For prototyping it can be useful, for example when dealing with
functions that haven't been implemented yet, but in those cases the more
descriptive `unimplemented` is better. In tests `panic` is a reasonable way to
explicitly fail.

The `Option` type is for when a value is optional or when the lack of a value is
not an error condition. For example the parent of a directory - `/` and `C:`
don't have one. When dealing with `Option`s, `unwrap` is fine for prototyping
and cases where it's absolutely certain that there is guaranteed to be a value.
However `expect` is more useful since it lets you specify an error message in
case something goes wrong anyway.

When there is a chance that things do go wrong and the caller has to deal with
the problem, use `Result`. You can `unwrap` and `expect` them as well (please
don't do that unless it's a test or quick prototype).

For a more rigorous discussion of error handling, refer to the error handling
section in the [official book][book].

[book]: https://doc.rust-lang.org/book/ch09-00-error-handling.html

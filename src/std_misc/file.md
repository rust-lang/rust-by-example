# File I/O

The `File` struct represents a file that has been opened (it wraps a file
descriptor), and gives read and/or write access to the underlying file.

Since many things can go wrong when doing file I/O, all the `File` methods
return the `io::Result<T>` type, which is an alias for `Result<T, io::Error>`.

This makes the failure of all I/O operations *explicit*. Thanks to this, the
programmer can see all the failure paths, and is encouraged to handle them in a
proactive manner.

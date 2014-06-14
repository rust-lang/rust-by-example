To link a crate to this new library, the `extern crate` declaration must be
used. This will not only link the library, but also import all its items
under a module named the same as the library. The visibility rules that apply
to modules also apply to libraries.

{executable.rs}

```
# The `-L .` argument adds the current directory to the library search path
$ rustc -L . executable.rs && ./executable
called erty's `public_function()`
called erty's `indirect_access()`, that
> called erty's `private_function()`
```

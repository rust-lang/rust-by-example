Rust provides a Foreign Function Interface (FFI) to C libraries. Foreign
functions must be declared inside an `extern` block annotated with a `#[link]`
attribute containing the name of the foreign library.

{ffi.rs}

{ffi.out}

Since calling foreign functions is considered unsafe, it's common to write safe
wrappers around them.

{safe.rs}

{safe.out}

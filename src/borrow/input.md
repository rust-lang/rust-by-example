Since Rust moves data by default, this means that passing an object to a
function will transfer ownership of the object to the function. Furthermore,
Rust enforces the `RAII` ("Resource Acquisition Is Initialization")
discipline, meaning that the object will be destroyed when it goes out of 
scope, following the previous example the object sent to the function will 
be destroyed over there.

{raii.rs}

This may not always be desirable, hence Rust provides a *borrow* mechanism.
Using this mechanism references to the object can be passed to functions
without giving up the ownership of the object.

Since the creation and destruction, i.e. the *lifetime*, of an object are known
to the compiler, it can enforce valid borrowing via its *borrow checker*.

{borrow.rs}

{borrow.out}

(`&` and `&mut` are the const correctness of Rust, but you will never forget to
 add a const to the function signature, since immutable is the default and the
 compiler will ~~yell at you~~ tell you if you forgot a `mut` somewhere)

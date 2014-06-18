Constants can be declared in the global scope using the `static` keyword, the
type annotation is obligatory in this case. These constants are placed in a
read-only section of the memory and can be accessed in any other part of the
program.

String literals like `"string"` can also be assigned to static variables. These
variables have type signature `&'static str`, and are references to strings
allocated in read-only memory. `'static` is a special lifetime that outlives
all the other lifetimes, and indicates that the referenced data is available in
all the scopes.

{constants.play}

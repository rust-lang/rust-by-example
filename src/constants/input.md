Constants can be declared in the global scope using the `static` keyword, the
type annotation is obligatory in this case. These constants are placed in a
read-only section of the memory and can be accessed in any other part of the
program.

Strings, which are normally heap allocated, can be constants too, but they
require the `'static` lifetime in their type annotation.

{constants.rs}

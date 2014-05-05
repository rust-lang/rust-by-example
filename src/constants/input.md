Constants can be declared in the global scope using the `static` keyword, the
type annotation is obligatory in this case. These constants are placed in a
read-only section of the memory and can be accessed in any other part of the
program.

Strings can be constants too, their definitions use `'static str` as type
signature. `'static` is an special lifetime that outlives all the other
lifetimes, it indicates that the referenced data is available in all the
scopes.

{constants.rs}

{constants.out}

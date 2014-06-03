Variables in Rust are declared using the `let` keyword. Variables are immutable
by default, but this can be overridden using the `mut` qualifier.

{variables.rs}

{variables.play}

The compiler will throw a detailed diagnostic about mutability errors.

{variables.out}

It's possible to declare variables first, and initialize them later, but this
form is seldom used. Also, the compiler forbids usage of uninitialized
variables, as this would lead to undefined behavior.

{declare.rs}

{declare.play}

{declare.out}

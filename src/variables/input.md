Variables in Rust are declared using the `let` keyword. Variables are immutable
by default and mutability can be specified using the `mut` modifier.

{variables.rs}

The compiler will throw a detailed diagnostic about mutability errors.

{variables.out}

It's possible to declare variables first, and initialize them later, but this
form is seldom used. Also, the compiler forbids usage of uninitialized
variables, as this would lead to undefined behavior.

{declare.rs}

{declare.out}

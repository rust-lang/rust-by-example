Rust provides a `loop` keyword to indicate an infinite loop.

The `break` keyword can be used to exit a loop at anytime, whereas the
`continue` can be used to skip the rest of the iteration and start a new one.

{loop.play}

It's possible to `break` or `continue` outer loops when dealing with nested
loops. In these cases, the loops must be annotated with some `'label` and the
label must be passed to the `break`/`continue` statement.

{nested.rs}

{nested.out}

The `while` keyword can be used for looping until a condition is met.

Let's write the infamous fizzbuzz using a `while` loop.

{while.rs}

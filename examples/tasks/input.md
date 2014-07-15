Rust provides a mechanism for spawning lightweight tasks via the `spawn`
function, the argument of this function is an owned closure named `proc`.

{tasks.play}

These tasks will be scheduled by the Rust runtime and the order of execution of
these tasks will be non-deterministic.

(By default, Rust uses its *native* runtime, which maps each Rust task to a
native thread. Rust also provides a *green* runtime that provides green threads
and maps M Rust tasks to N native threads.)

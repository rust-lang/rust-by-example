Rust provides a mechanism for spawning native OS threads via the `spawn`
function, the argument of this function is a moving closure.

{threads.play}

These threads will be scheduled by the OS and the order of execution of
these tasks will be non-deterministic.

(Currently Rust uses *native* runtime, which maps each Rust task to a
native thread.)

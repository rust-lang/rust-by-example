Rust provides a mechanism for spawning lightweight tasks via the `spawn`
function, the argument of this function is a owned closure named `proc`.

{tasks.rs}

These tasks will be scheduled by the Rust runtime and the order of execution of
these tasks will be non-deterministic.

``` bash
$ rustc tasks.rs && ./tasks
this is task number 2
this is task number 1
this is task number 4
this is task number 3
this is task number 0
this is task number 6
this is task number 5
this is task number 8
this is task number 9
this is task number 7
```

(By default, Rust uses it's *native* runtime, which maps each rust task to a
native thread. Rust also provides a *green* runtime that provides green threads
and maps M Rust tasks to N native threads)

The `Process` struct represents a running or finished child process. And the
`Command` struct is a process builder.

{process.rs}

{process.play}

{process.out}

(You are encouraged to try the previous example with an incorrect flag)

`Process` exposes the `stdin`, `stdout` and `stderr` handles for interaction
with the child process via pipes.

{pipe.rs}

{pipe.play}

{pipe.out}

When a `Process` that owns a resource (a *runnig* child process) goes out of
scope the task will *wait* until the child process finishes before releasing
the resource.

{wait.rs}

```
$ rustc wait.rs && ./wait
reached end of main
# `wait` keeps running for 5 seconds
# `sleep 5` command ends, and then our `wait` program finishes
```

Be sure to check the other methods of
[`Command`](http://static.rust-lang.org/doc/master/std/io/process/struct.Command.html)
that allow changing the environment and the working directory of the spawned
process. The
[`Process`](http://static.rust-lang.org/doc/master/std/io/process/struct.Process.html)
struct also provides several methods to send signals to the child process.

# 40 Tasks

Rust provides a mechanism for spawning lightweight tasks via the `spawn`
function, the argument of this function is an owned closure named `proc`.

<div id="active-code">
<button class="btn btn-primary" type="button" id="run-code">Run</button>
<button class="btn btn-primary" type="button" id="reset-code">Reset</button>
<div id="editor">static NTASKS: int = 10;

// This is the &#96;main&#96; task
fn main() {
    for i in range(0, NTASKS) {
        // Spin up another task
        spawn(proc() {
            println!("this is task number {}", i)
        });
    }
}</div>
<div id="result"></div>
</div>

These tasks will be scheduled by the Rust runtime and the order of execution of
these tasks will be non-deterministic.

(By default, Rust uses its *native* runtime, which maps each Rust task to a
native thread. Rust also provides a *green* runtime that provides green threads
and maps M Rust tasks to N native threads.)

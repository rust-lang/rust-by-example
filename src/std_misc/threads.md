# Threads

Rust provides a mechanism for spawning native OS threads via the `spawn`
function, the argument of this function is a moving closure.

```rust,editable
use std::thread;

const NTHREADS: u32 = 10;

// This is the `main` thread
fn main() {
    // Make a vector to hold the children which are spawned.
    let mut children = vec![];

    for i in 0..NTHREADS {
        // Spin up another thread
        children.push(thread::spawn(move || {
            println!("this is thread number {}", i);
        }));
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
}
```

These threads will be scheduled by the OS.

## Scoped threads

`thread::spawn` requires its closure to own everything it captures
(`'static`), which forces awkward moves of stack data. `thread::scope`
(stable since 1.63) lifts that restriction: threads spawned inside the
scope may borrow stack data, because the scope joins every thread
before returning.

```rust,editable
use std::thread;

fn main() {
    let numbers = vec![1, 2, 3, 4];

    thread::scope(|s| {
        // Borrow `numbers` instead of moving it: the scope guarantees
        // both threads finish before `numbers` is used again below.
        s.spawn(|| println!("len: {}", numbers.len()));
        s.spawn(|| println!("sum: {}", numbers.iter().sum::<i32>()));
    });

    println!("still ours: {:?}", numbers);
}
```

## `Send` and `Sync`

`Send` means a value may be moved to another thread; `Sync` means a
shared reference to it may be used from another thread. Most types are
both, and the compiler checks the bounds automatically when you spawn
or share. `Rc` is the classic exception: it is neither `Send` nor
`Sync`, which is why shared ownership across threads needs `Arc`.

```rust,ignore
use std::rc::Rc;
use std::thread;

fn main() {
    let shared = Rc::new(42);
    // Error: `Rc<i32>` cannot be sent between threads safely.
    thread::spawn(move || println!("{}", shared));
}
```

### Exercise: Fix a borrow across threads

The program below does not compile. Press Run, read the compiler error, then fix it.

Task: Make the spawned thread see `numbers` without moving it.

```rust,editable,ignore,mdbook-runnable
// BROKEN: press Run to see the compiler error, then fix the program.
use std::thread;

fn main() {
    let numbers = vec![1, 2, 3, 4];

    // Error: `thread::spawn` requires ownership, but `numbers` is borrowed
    // and used again below.
    let handle = thread::spawn(|| {
        println!("sum: {}", numbers.iter().sum::<i32>());
    });
    handle.join().unwrap();

    println!("still ours: {:?}", numbers);
}
```

<details><summary>Hint</summary>

Spawned threads may borrow stack data when a scope guarantees they finish first.

</details>

<details><summary>Solution</summary>

```rust,editable
use std::thread;

fn main() {
    let numbers = vec![1, 2, 3, 4];

    // The scope joins the thread before returning, so borrowing is safe.
    thread::scope(|s| {
        s.spawn(|| println!("sum: {}", numbers.iter().sum::<i32>()));
    });

    println!("still ours: {:?}", numbers);
}
```

</details>

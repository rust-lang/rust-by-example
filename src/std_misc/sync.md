# Shared state: `Mutex`, `RwLock`, atomics

When threads must mutate shared data, the standard library offers three
levels of protection. `Mutex` allows one thread at a time and blocks
the rest; `RwLock` allows many concurrent readers or one writer;
atomics such as `AtomicUsize` update a single integer without locking
at all.

`Mutex::lock` returns a guard that dereferences to the data. The lock
releases when the guard drops, so keep the critical section short.
A lock can be *poisoned* if another thread panicked while holding it;
`lock().unwrap()` is fine for examples, and production code decides
whether the guarded data is still usable.

```rust,editable
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            // The guard drops at the end of this statement, ending the
            // critical section as early as possible.
            *counter.lock().unwrap() += 1;
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("counter: {}", *counter.lock().unwrap());
}
```

`RwLock` splits the guard in two: `read` for shared access, `write`
for exclusive access. It pays off when reads dominate and are slow
enough to overlap usefully.

```rust,editable
use std::sync::RwLock;

fn main() {
    let scores = RwLock::new(vec![1, 2, 3]);

    // Many readers can hold this guard at once.
    println!("sum: {}", scores.read().unwrap().iter().sum::<i32>());

    // Only one writer, and no readers while it is held.
    scores.write().unwrap().push(4);
    println!("after: {:?}", scores.read().unwrap());
}
```

For a bare counter, an atomic skips the lock entirely.
`fetch_add` updates the value as one indivisible step. The `Ordering`
argument controls how the operation synchronizes with other threads;
`SeqCst` is the strictest and the right default unless profiling plus
expert review says otherwise (see the `std::sync::atomic` docs for the
full memory model).

```rust,editable
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    let hits = AtomicUsize::new(0);

    hits.fetch_add(1, Ordering::SeqCst);
    hits.fetch_add(1, Ordering::SeqCst);

    println!("hits: {}", hits.load(Ordering::SeqCst));
}
```

### See also:

[`std::sync::Mutex`][mutex], [`std::sync::RwLock`][rwlock],
[`std::sync::atomic`][atomic], and [Threads][threads].

[mutex]: https://doc.rust-lang.org/std/sync/struct.Mutex.html
[rwlock]: https://doc.rust-lang.org/std/sync/struct.RwLock.html
[atomic]: https://doc.rust-lang.org/std/sync/atomic/index.html
[threads]: threads.md

### Exercise: Convert a counter to an atomic

Task: Convert the `Arc<Mutex<usize>>` counter below to `Arc<AtomicUsize>`.

<details><summary>Hint</summary>

Share the atomic through the threads and bump it with one operation that needs no guard or lock.

</details>

<details><summary>Solution</summary>

```rust,editable
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

fn main() {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            counter.fetch_add(1, Ordering::SeqCst);
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    assert_eq!(counter.load(Ordering::SeqCst), 10);
    println!("counter: {}", counter.load(Ordering::SeqCst));
}
```
</details>

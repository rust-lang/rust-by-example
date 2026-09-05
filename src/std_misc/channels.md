# Channels

Rust provides asynchronous `channels` for communication between threads. Channels
allow a unidirectional flow of information between two end-points: the
`Sender` and the `Receiver`.

```rust,editable
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

static NTHREADS: i32 = 3;

fn main() {
    // Channels have two endpoints: the `Sender<T>` and the `Receiver<T>`,
    // where `T` is the type of the message to be transferred
    // (type annotation is superfluous)
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    let mut children = Vec::new();

    for id in 0..NTHREADS {
        // The sender endpoint can be copied
        let thread_tx = tx.clone();

        // Each thread will send its id via the channel
        let child = thread::spawn(move || {
            // The thread takes ownership over `thread_tx`
            // Each thread queues a message in the channel
            thread_tx.send(id).unwrap();

            // Sending is a non-blocking operation, the thread will continue
            // immediately after sending its message
            println!("thread {} finished", id);
        });

        children.push(child);
    }

    // Here, all the messages are collected
    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        // The `recv` method picks a message from the channel
        // `recv` will block the current thread if there are no messages available
        ids.push(rx.recv());
    }

    // Wait for the threads to complete any remaining work
    for child in children {
        child.join().expect("oops! the child thread panicked");
    }

    // Show the order in which the messages were sent
    println!("{:?}", ids);
}
```

### Exercise: Sum the thread ids

Task: Edit the example above to sum the received ids and assert the total.

<details><summary>Hint</summary>

Each thread sends its own number, so the collected values always add up to the same total.

</details>

<details><summary>Solution</summary>

```rust,editable
use std::sync::mpsc;
use std::thread;

static NTHREADS: i32 = 3;

fn main() {
    let (tx, rx) = mpsc::channel();

    for id in 0..NTHREADS {
        let thread_tx = tx.clone();
        thread::spawn(move || {
            thread_tx.send(id).unwrap();
        });
    }
    // The original sender is no longer needed; dropping it lets
    // the channel close once every thread finishes.
    drop(tx);

    let mut sum = 0;
    for _ in 0..NTHREADS {
        sum += rx.recv().unwrap();
    }

    assert_eq!(sum, 0 + 1 + 2);
    println!("sum of ids: {}", sum);
}
```

</details>

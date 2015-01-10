use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread::Thread;

static NTHREADS: usize = 3;

fn main() {
    // Channels have two endpoints: the `Sender<T>` and the `Receiver<T>`,
    // where `T` is the type of the message to be transfer
    // (type annotation is superfluous)
    let (tx, rx): (Sender<usize>, Receiver<usize>) = mpsc::channel();

    for id in range(0, NTHREADS) {
        // The sender endpoint can be copied
        let thread_tx = tx.clone();

        // Each thread will send its id via the channel
        Thread::spawn(move || {
            // The thread takes ownership over `thread_tx`
            // Each thread queues a message in the channel
            thread_tx.send(id).unwrap();

            // Sending is a non-blocking operation, the thread will continue
            // immediately after sending its message
            println!("thread {} finished", id);
        });
    }

    // Here, all the messages are collected
    let mut ids = Vec::with_capacity(NTHREADS);
    for _ in range(0, NTHREADS) {
        // The `recv` method picks a message from the channel
        // `recv` will block the current thread if there no messages available
        ids.push(rx.recv());
    }

    // Show the order in which the messages were sent
    println!("{:?}", ids);
}

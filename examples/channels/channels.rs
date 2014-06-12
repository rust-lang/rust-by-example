use std::comm;

static NTASKS: uint = 3;

fn main() {
    // Channels have two endpoints: the `Sender<T>` and the `Receiver<T>`,
    // where `T` is the type of the message to be transfer
    // (type annotation is superfluous)
    let (tx, rx): (Sender<uint>, Receiver<uint>) = comm::channel();

    for id in range(0, NTASKS) {
        // The sender endpoint can be copied
        let task_tx = tx.clone();

        // Each task will send its id via the channel
        spawn(proc() {
            // The task takes ownership over `task_tx`
            // Each task queues a message in the channel
            task_tx.send(id);

            // Sending is a non-blocking operation, the task will continue
            // immediately after sending its message
            println!("task {} finished", id);
        });
    }

    // Here, all the messages are collected
    let mut ids = Vec::with_capacity(NTASKS);
    for _ in range(0, NTASKS) {
        // The `recv` method picks a message from the channel
        // `recv` will block the current task if there no messages available
        ids.push(rx.recv());
    }

    // Show the order in which the messages were sent
    println!("{}", ids);
}

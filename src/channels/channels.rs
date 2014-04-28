use std::comm::channel;

static NTASKS: int = 5;

fn main() {
    // channels have two endpoints: the Sender<T> and the Receiver<T>,
    // where T is the type of the message to be transfer
    // (type annotation is superfluous)
    let (tx, rx): (Sender<_>, Receiver<_>) = channel();

    for id in range(0, NTASKS) {
        // the sender endpoint can be copied
        let tx = tx.clone();

        // each task will send its id via the channel
        spawn(proc() {
            // queue message in the channel
            tx.send(id);

            // sending is a non-blocking operation, the task will continue
            // immediately after sending its message
            println!("task number {} finished", id);
        });
    }

    // here, all the messages are collected
    for _ in range(0, NTASKS) {
        // the recv() methods picks a message from the channel
        let id = rx.recv();

        println!("task number {} reported", id);
    }

    // receiving blocks the task if there is no message available, until a
    // new message arrives
    rx.recv();

    println!("this point will never be reached!");
}

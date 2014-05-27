use std::comm::channel;

static NMESSAGES: int = 5;

fn main() {
    let (tx, rx) = channel();

    for i in range(0, NMESSAGES) {
        tx.send(i);
    }

    for _ in range(0, NMESSAGES) {
        println!("{}", rx.recv());
    }
}

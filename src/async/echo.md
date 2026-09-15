# Capstone: fan-out chat

This page combines everything from the Async chapter: two client tasks
send messages through one `mpsc` channel while the main task receives
until every sender is gone. No `join!` tracks the clients — dropping
the last `Sender` (including the original) closes the channel, and the
`None` from `recv()` is the shutdown signal.

```rust,editable,ignore,mdbook-runnable
// Cargo.toml: tokio = { version = "1", features = ["full"] }

use tokio::sync::mpsc;

async fn client(id: u32, tx: mpsc::Sender<String>) {
    for round in 1..=3 {
        tx.send(format!("client {id} round {round}")).await.unwrap();
    }
    // `tx` drops here; when the last sender drops, the channel closes.
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(16);

    tokio::spawn(client(1, tx.clone()));
    tokio::spawn(client(2, tx.clone()));
    // The original sender is not needed anymore: the channel now closes
    // exactly when both clients finish.
    drop(tx);

    let mut received = 0;
    while let Some(msg) = rx.recv().await {
        println!("got: {msg}");
        received += 1;
    }

    assert_eq!(received, 6);
    println!("all clients done");
}
```

The pattern generalizes: senders are producers, the receiver loop is
the consumer, and closing is structural rather than messaged. Adding a
producer never touches the shutdown logic.

### See also:

[Spawning tasks and channels](spawn.md) and [Streams of
messages](streams.md).

### Exercise: Add a third client

Task: Add a third client task and update the assertion to match.

<details><summary>Hint</summary>

Every client sends the same number of messages, so the total scales directly with the client count.

</details>

<details><summary>Solution</summary>

```rust,editable,ignore,mdbook-runnable
// Cargo.toml: tokio = { version = "1", features = ["full"] }

use tokio::sync::mpsc;

async fn client(id: u32, tx: mpsc::Sender<String>) {
    for round in 1..=3 {
        tx.send(format!("client {id} round {round}")).await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(16);

    tokio::spawn(client(1, tx.clone()));
    tokio::spawn(client(2, tx.clone()));
    tokio::spawn(client(3, tx.clone()));
    drop(tx);

    let mut received = 0;
    while let Some(msg) = rx.recv().await {
        println!("got: {msg}");
        received += 1;
    }

    assert_eq!(received, 9);
    println!("all clients done");
}
```
</details>

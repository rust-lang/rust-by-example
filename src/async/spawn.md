# Spawning tasks and channels

`#[tokio::main]` turns `main` into an async entry point by building a
runtime and blocking on the future. Inside it, `tokio::spawn` runs a
future as a background task and returns a `JoinHandle`; awaiting the
handle yields the task's return value. `tokio::join!` waits for several
futures at once without spawning.

```rust,editable,ignore,mdbook-runnable
// Cargo.toml: tokio = { version = "1", features = ["full"] }

async fn work(id: u32) -> u32 {
    id * 10
}

#[tokio::main]
async fn main() {
    // Run in the background; `handle` is a future for its result.
    let handle = tokio::spawn(work(1));
    // Run two futures concurrently on this task and wait for both.
    let (a, b) = tokio::join!(work(2), work(3));

    println!("spawned: {}", handle.await.unwrap());
    println!("joined: {} {}", a, b);
}
```

Tasks communicate through channels. An `mpsc` (multi-producer,
single-consumer) channel clones its sender across tasks while one
receiver collects; `send().await` applies backpressure when the buffer
is full.

```rust,editable,ignore,mdbook-runnable
// Cargo.toml: tokio = { version = "1", features = ["full"] }

use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(8);

    tokio::spawn(async move {
        tx.send("hello").await.unwrap();
        tx.send("world").await.unwrap();
        // `tx` drops here, closing the channel.
    });

    while let Some(msg) = rx.recv().await {
        println!("got: {}", msg);
    }
}
```

### See also:

[`tokio::spawn`][spawn], [`tokio::join!`][join], and
[`tokio::sync::mpsc`][mpsc].

[spawn]: https://docs.rs/tokio/latest/tokio/task/fn.spawn.html
[join]: https://docs.rs/tokio/latest/tokio/macro.join.html
[mpsc]: https://docs.rs/tokio/latest/tokio/sync/mpsc/index.html

### Exercise: Spawn two tasks and join both

Task: Spawn a second task beside the first and wait for both with `join!`.

<details><summary>Hint</summary>

Each spawned task hands back a handle that can be awaited like any other future.

</details>

<details><summary>Solution</summary>

```rust,editable,ignore,mdbook-runnable
// Cargo.toml: tokio = { version = "1", features = ["full"] }

async fn work(id: u32) -> u32 {
    id * 10
}

#[tokio::main]
async fn main() {
    let first = tokio::spawn(work(1));
    let second = tokio::spawn(work(2));

    let (a, b) = tokio::join!(first, second);
    println!("results: {} {}", a.unwrap(), b.unwrap());
}
```
</details>

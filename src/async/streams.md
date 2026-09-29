# Streams of messages

A channel receiver used in a loop is the simplest async stream: each
`recv().await` yields the next message, and `None` ends the loop when
all senders drop. No extra crate is needed beyond tokio — think of it
as the async counterpart of iterating, where waiting replaces blocking.

```rust,editable,ignore,mdbook-runnable
// Cargo.toml: tokio = { version = "1", features = ["full"] }

use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(8);

    tokio::spawn(async move {
        for n in 1..=5 {
            tx.send(n * n).await.unwrap();
        }
        // All `tx` clones drop here, so the stream ends.
    });

    // Each iteration waits for the next message without blocking a thread.
    let mut total = 0;
    while let Some(n) = rx.recv().await {
        total += n;
    }
    println!("sum of squares: {}", total);
}
```

Shutting the stream down is structural: dropping the last sender closes
it, and the receiver loop exits by itself. For richer combinators
(`map`, `filter`, `merge`), the [`tokio-stream`][tokio_stream] crate
wraps receivers in a `StreamExt` API that mirrors synchronous
iterators.

### See also:

[`tokio::sync::mpsc`][mpsc], [`tokio-stream`][tokio_stream], and
[Spawning](spawn.md).

[mpsc]: https://docs.rs/tokio/latest/tokio/sync/mpsc/index.html
[tokio_stream]: https://docs.rs/tokio-stream/latest/tokio_stream/

### Exercise: Sum a stream with a cutoff

Task: Stop consuming the stream once the running total exceeds ten and report the count.

<details><summary>Hint</summary>

The receive loop is ordinary control flow, so a threshold check inside it can break out early.

</details>

<details><summary>Solution</summary>

```rust,editable,ignore,mdbook-runnable
// Cargo.toml: tokio = { version = "1", features = ["full"] }

use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(8);

    tokio::spawn(async move {
        for n in 1..=10 {
            if tx.send(n).await.is_err() {
                break;
            }
        }
    });

    let mut total = 0;
    let mut count = 0;
    while let Some(n) = rx.recv().await {
        total += n;
        count += 1;
        if total > 10 {
            break;
        }
    }
    println!("stopped after {} values, total {}", count, total);
}
```
</details>

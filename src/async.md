# Async

Synchronous code blocks the thread while waiting: a network read parks
the whole thread until bytes arrive. Asynchronous code instead expresses
waiting as a *future* — a value that is lazy (it does nothing until
polled) and that a *runtime* drives to completion, freeing the thread
to run other futures in the meantime.

Rust's standard library provides the core pieces (`Future`, `async`,
`.await`), but no runtime and no timers or network drivers. This
chapter uses [`tokio` 1.x][tokio] as its runtime, the most widely used
choice. Every snippet that needs tokio is marked `ignore` (it is not
compiled by `mdbook test`) and carries its dependency as a comment
header:

```rust,ignore
// Cargo.toml: tokio = { version = "1", features = ["full"] }

#[tokio::main]
async fn main() {
    println!("hello from async main");
}
```

The three leaves cover [`await` syntax](async/await_syntax.md),
[spawning and channels](async/spawn.md), and [message
streams](async/streams.md). To run any of them locally, create a binary
crate, add the `Cargo.toml` line from its header, and paste the body
into `src/main.rs`.

[tokio]: https://docs.rs/tokio/latest/tokio/

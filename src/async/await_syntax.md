# `async` and `.await`

An `async fn` does not run its body when called: it returns a future,
and the body runs only once the future is `.await`ed. `.await` pauses
the current future (yielding the thread back to the runtime) until the
awaited future completes, then resumes with its value. `async` blocks
work the same way for inline futures.

```rust,editable,ignore,mdbook-runnable
// Cargo.toml: tokio = { version = "1", features = ["full"] }

async fn greet(name: &str) -> String {
    // Calling `greet` only builds the future; `.await` runs it.
    format!("hello {}", name)
}

#[tokio::main]
async fn main() {
    let message = greet("ferris").await;
    println!("{}", message);

    let ready = async {
        40 + 2
    };
    println!("inline: {}", ready.await);
}
```

Values held across `.await` must survive a suspension, so the compiler
requires them to be `Send` when the runtime may resume the future on
another thread. The classic footgun is holding a `MutexGuard` across
an await: the lock stays held while the task is parked, blocking every
other task that wants it. Drop the guard (or copy out what you need)
before awaiting.

```rust,editable,ignore,mdbook-runnable
// Cargo.toml: tokio = { version = "1", features = ["full"] }

use std::sync::Mutex;

async fn fetch_cached(cache: &Mutex<String>) -> String {
    // Copy the data out so the guard drops before any `.await`.
    let cached = cache.lock().unwrap().clone();
    if !cached.is_empty() {
        return cached;
    }

    let fresh = slow_network_call().await;
    *cache.lock().unwrap() = fresh.clone();
    fresh
}

async fn slow_network_call() -> String {
    "fresh-data".to_string()
}

#[tokio::main]
async fn main() {
    let cache = Mutex::new(String::new());
    println!("first: {}", fetch_cached(&cache).await);
    println!("cached: {}", fetch_cached(&cache).await);
}
```

### See also:

[`std::future`][future], [`tokio::task`][task], and [Spawning](spawn.md).

[future]: https://doc.rust-lang.org/std/future/index.html
[task]: https://docs.rs/tokio/latest/tokio/task/index.html

### Exercise: Add a second awaited call

Task: Extend the program with a second `async fn` and await both results in order.

<details><summary>Hint</summary>

Each call builds an independent future, so awaiting them one after another runs each body in turn.

</details>

<details><summary>Solution</summary>

```rust,editable,ignore,mdbook-runnable
// Cargo.toml: tokio = { version = "1", features = ["full"] }

async fn greet(name: &str) -> String {
    format!("hello {}", name)
}

async fn farewell(name: &str) -> String {
    format!("goodbye {}", name)
}

#[tokio::main]
async fn main() {
    let hello = greet("ferris").await;
    let bye = farewell("ferris").await;
    println!("{} and {}", hello, bye);
}
```
</details>

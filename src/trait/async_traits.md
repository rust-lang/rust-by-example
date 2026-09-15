# `async` in traits

Since Rust 1.75, traits may declare `async fn` directly — no macro or
manual `Future` boxing needed. Each implementation's future captures
the receiver and arguments, exactly as if the method returned
`impl Future<Output = ...> + '_`.

```rust,editable
trait Notifier {
    async fn notify(&self, msg: &str) -> String;
}

struct Console;

impl Notifier for Console {
    async fn notify(&self, msg: &str) -> String {
        format!("notified: {}", msg)
    }
}

fn main() {
    // Calling `notify` only builds the future; running it needs a
    // runtime (see below). This checks the trait shape compiles and
    // that the future can be named and passed around.
    fn returns_future<N: Notifier>(notifier: N) {
        let _pending = notifier.notify("hi");
    }
    returns_future(Console);
    println!("trait with async fn compiles");
}
```

Because the future borrows `&self`, runtimes that move futures across
threads need it to be `Send`. Add an explicit bound where that matters:
`async fn run(&self) -> T where Self: Sync`, or require the future
itself via `fn spawn<N: Notifier + Sync + 'static>(n: N)`. The compiler
tells you when the bound is missing.

```rust,editable,ignore,mdbook-runnable
// Cargo.toml: tokio = { version = "1", features = ["full"] }

trait Notifier {
    async fn notify(&self, msg: &str) -> String;
}

struct Console;

impl Notifier for Console {
    async fn notify(&self, msg: &str) -> String {
        format!("notified: {}", msg)
    }
}

#[tokio::main]
async fn main() {
    let console = Console;
    println!("{}", console.notify("ferris").await);
}
```

### See also:

[`async` and `.await`][async_await] and [Static and dynamic
dispatch][dispatch].

[async_await]: ../async/await_syntax.md
[dispatch]: https://doc.rust-lang.org/book/ch17-02-trait-objects.html

### Exercise: Add a defaulted trait method

Task: Add a second method with a default body that calls the first.

<details><summary>Hint</summary>

A default body runs like any other async block, so it can await calls on the same receiver.

</details>

<details><summary>Solution</summary>

```rust,editable
trait Notifier {
    async fn notify(&self, msg: &str) -> String;

    async fn notify_twice(&self, msg: &str) -> String {
        let first = self.notify(msg).await;
        let second = self.notify(msg).await;
        format!("{}\n{}", first, second)
    }
}

struct Console;

impl Notifier for Console {
    async fn notify(&self, msg: &str) -> String {
        format!("notified: {}", msg)
    }
}

fn main() {
    fn returns_future<N: Notifier>(notifier: N) {
        let _pending = notifier.notify_twice("hi");
    }
    returns_future(Console);
    println!("default method compiles");
}
```
</details>

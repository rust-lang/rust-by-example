# `OnceLock` and `LazyLock`

`OnceLock<T>` stores a value that is initialized at most once, even when
several threads race to initialize it. `get_or_init` runs its closure
only for the winner; every caller gets a reference to the single value.

```rust,editable
use std::sync::OnceLock;

static CONFIG: OnceLock<String> = OnceLock::new();

fn config() -> &'static str {
    CONFIG.get_or_init(|| {
        println!("initializing once");
        "cfg-v1".to_string()
    })
}

fn main() {
    println!("first: {}", config());
    // The message above prints only once: this call reuses the value.
    println!("second: {}", config());
}
```

`LazyLock<T>` (stable since 1.80) wraps the same idea for statics: the
closure runs on first dereference, so declaration and initialization
stay in one place.

```rust,editable
use std::sync::LazyLock;

static GREETING: LazyLock<String> =
    LazyLock::new(|| format!("hello {}", "ferris"));

fn main() {
    // Nothing above ran the closure; this first use does.
    println!("{}", *GREETING);
    println!("again: {}", *GREETING);
}
```

### See also:

[`std::sync::OnceLock`][oncelock] and [`std::sync::LazyLock`][lazylock].

[oncelock]: https://doc.rust-lang.org/std/sync/struct.OnceLock.html
[lazylock]: https://doc.rust-lang.org/std/sync/struct.LazyLock.html

### Exercise: Predict which threads initialize

Task: Predict how many times the initializer prints when ten threads share one `OnceLock`, then check your answer.

<details><summary>Hint</summary>

Only one thread can win the race to fill the cell; the rest wait and share the result.

</details>

<details><summary>Solution</summary>

```rust,editable
use std::sync::OnceLock;
use std::thread;

static CELL: OnceLock<String> = OnceLock::new();

fn main() {
    let handles: Vec<_> = (0..10)
        .map(|_| {
            thread::spawn(|| {
                CELL.get_or_init(|| {
                    println!("initialized");
                    "shared".to_string()
                })
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }

    // "initialized" prints exactly once no matter how many threads raced.
    assert_eq!(CELL.get().unwrap(), "shared");
}
```
</details>

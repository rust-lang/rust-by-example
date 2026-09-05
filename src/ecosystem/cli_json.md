# CLI + JSON walkthrough

This page combines [`clap`][clap] (argument parsing via derive) with
[`serde_json`][serde_json] (serialization) into one applied program: a
greeter CLI that prints its greeting as JSON. It builds on [Procedural
macros][proc] for the derives and [Error
reporting][reporting] for the `anyhow` error handling.

```rust,editable,ignore,mdbook-runnable
// Cargo.toml: clap = { version = "4", features = ["derive"] }
// Cargo.toml: serde = { version = "1", features = ["derive"] }
// Cargo.toml: serde_json = "1"
// Cargo.toml: anyhow = "1"

use anyhow::{Context, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};

/// Greet someone and print the greeting as JSON.
#[derive(Parser, Debug)]
#[command(name = "greeter", version, about)]
struct Args {
    /// Who to greet.
    #[arg(long, default_value = "world")]
    name: String,

    /// Uppercase the greeting.
    #[arg(long)]
    shout: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Greeting {
    message: String,
}

fn greet(args: &Args) -> Greeting {
    let mut message = format!("hello {}", args.name);
    if args.shout {
        message.make_ascii_uppercase();
    }
    Greeting { message }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let greeting = greet(&args);

    // Round-trip through JSON to prove the shape survives serialization.
    let json = serde_json::to_string(&greeting)?;
    let back: Greeting = serde_json::from_str(&json)
        .context("serializing the greeting produced invalid JSON")?;
    assert_eq!(greeting, back);

    println!("{}", json);
    Ok(())
}
```

```shell
$ cargo run -- --name ferris
{"message":"hello ferris"}
$ cargo run -- --name ferris --shout
{"message":"HELLO FERRIS"}
```

`#[derive(Parser)]` generates the `--help` text, validation, and
`--version` from the struct definition, so adding a flag is adding a
field. `#[derive(Serialize, Deserialize)]` does the same for the JSON
shape: rename a field once and both directions follow.

### See also:

[Procedural macros][proc], [Error reporting][reporting], and
[Workspaces][workspaces].

[proc]: ../macros/proc.md
[reporting]: ../error/reporting.md
[workspaces]: ../cargo/workspaces.md
[clap]: https://docs.rs/clap/latest/clap/
[serde_json]: https://docs.rs/serde_json/latest/serde_json/

### Exercise: Add a repeat flag

Task: Add a `--repeat N` flag that prints the greeting JSON N times, one per line.

<details><summary>Hint</summary>

A numeric field with a default behaves like the existing name field, and the print can move into a counted loop.

</details>

<details><summary>Solution</summary>

```rust,editable,ignore,mdbook-runnable
// Cargo.toml: clap = { version = "4", features = ["derive"] }
// Cargo.toml: serde = { version = "1", features = ["derive"] }
// Cargo.toml: serde_json = "1"
// Cargo.toml: anyhow = "1"

use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(name = "greeter", version, about)]
struct Args {
    #[arg(long, default_value = "world")]
    name: String,

    #[arg(long)]
    shout: bool,

    /// How many times to print the greeting.
    #[arg(long, default_value_t = 1)]
    repeat: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Greeting {
    message: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut message = format!("hello {}", args.name);
    if args.shout {
        message.make_ascii_uppercase();
    }
    let json = serde_json::to_string(&Greeting { message })?;
    for _ in 0..args.repeat {
        println!("{}", json);
    }
    Ok(())
}
```
</details>

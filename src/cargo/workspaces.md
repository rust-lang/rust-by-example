# Workspaces, features, and profiles

Real projects outgrow a single crate. A *workspace* groups related
crates under one `Cargo.toml` so they share a lockfile and build
together, while *features* make optional functionality opt-in and
*profiles* tune compiler settings per build mode.

```toml
# Cargo.toml at the workspace root. This package is virtual: it only
# organizes members, it produces no binary or library itself.
[workspace]
members = ["cli", "core"]
resolver = "2"

[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
```

```toml
# cli/Cargo.toml: features let users pay only for what they enable.
[package]
name = "cli"
version = "0.1.0"
edition = "2024"

[dependencies]
core = { path = "../core" }
serde_json = "1"
# Optional: only built when the `tls` feature is requested.
tokio-tls = { version = "0.1", optional = true }

[features]
default = []
tls = ["dep:tokio-tls"]
```

```shell
$ cargo build -p cli
$ cargo run -p cli --features tls -- --name ferris
$ cargo test --workspace
```

```toml
# Release tuning lives in profiles, not in code.
[profile.release]
opt-level = 3
lto = true
```

`Cargo.lock` pins the exact versions used; commit it for binaries (so
every checkout builds identically) and leave it out of libraries (so
downstream resolves fresh). `cargo update -p some-crate` bumps one
dependency deliberately instead of everything at once.

### See also:

[Dependencies](deps.md), [Conventions](conventions.md), and [the Cargo
Book on workspaces][workspaces].

[workspaces]: https://doc.rust-lang.org/cargo/reference/workspaces.html

### Exercise: Add an optional feature

Task: Add an opt-in `json` feature that pulls in `serde_json` only when requested.

<details><summary>Hint</summary>

Marking a dependency optional creates a same-named feature automatically, which other settings can then build on.

</details>

<details><summary>Solution</summary>

```toml
[dependencies]
serde_json = { version = "1", optional = true }

[features]
default = []
# Enables `serde_json` only for users who ask for `--features json`.
json = ["dep:serde_json"]
```

and gate the code behind the feature:

```rust,ignore
#[cfg(feature = "json")]
fn export_json() -> String {
    serde_json::to_string(&42).unwrap()
}

#[cfg(not(feature = "json"))]
fn export_json() -> String {
    "json support not compiled in".to_string()
}
```
</details>

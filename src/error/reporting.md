# Error reporting with `anyhow` and `thiserror`

Hand-written error enums (like [`DoubleError`][double]) teach the
mechanics, but production code splits the job in two: libraries define
errors with [`thiserror`][thiserror], binaries report them with
[`anyhow`][anyhow]. Both are `ignore` examples — add the `Cargo.toml`
line and run them locally.

A library derives `thiserror::Error` instead of writing `Display` and
`From` by hand. Each variant documents its own message, and `#[from]`
generates the conversion from an underlying error:

```rust,editable,ignore,mdbook-runnable
// Cargo.toml: thiserror = "2"

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseDoubleError {
    #[error("please use a vector with at least one element")]
    EmptyVec,
    #[error("could not parse the first element: {0}")]
    BadParse(#[from] std::num::ParseIntError),
}

fn double_first(vec: Vec<&str>) -> Result<i32, ParseDoubleError> {
    let first = vec.first().ok_or(ParseDoubleError::EmptyVec)?;
    Ok(2 * first.parse::<i32>()?)
}

fn main() {
    println!("{:?}", double_first(vec!["42"]));
    println!("{:?}", double_first(vec![]));
    println!("{:?}", double_first(vec!["tofu"]));
}
```

A binary returns `anyhow::Result` and adds context at each layer with
`.context()`. The printed report then reads like a backtrace of *what
the program was trying to do*, not just the low-level failure:

```rust,editable,ignore,mdbook-runnable
// Cargo.toml: anyhow = "1"

use anyhow::{Context, Result};

fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = vec
        .first()
        .context("expected at least one number on the command line")?;
    let n: i32 = first
        .parse()
        .with_context(|| format!("could not parse '{}' as a number", first))?;
    Ok(2 * n)
}

fn main() -> Result<()> {
    println!("doubled: {}", double_first(vec!["21"])?);
    Ok(())
}
```

Rule of thumb: `thiserror` for errors you *define* (libraries),
`anyhow` for errors you *propagate and report* (binaries and tests).

### See also:

[Defining an error type][double], [`thiserror`][thiserror], and
[`anyhow`][anyhow].

[double]: multiple_error_types/define_error_type.md
[thiserror]: https://docs.rs/thiserror/latest/thiserror/
[anyhow]: https://docs.rs/anyhow/latest/anyhow/

### Exercise: Derive the hand-written error

Task: Convert the hand-written `DoubleError` into a `thiserror` enum and compare line counts.

<details><summary>Hint</summary>

One attribute on each variant replaces a manual trait implementation and its conversions.

</details>

<details><summary>Solution</summary>

```rust,editable,ignore,mdbook-runnable
// Cargo.toml: thiserror = "2"

use thiserror::Error;

// Three lines replace the struct plus the manual `Display` impl,
// and `#[from]` adds `ParseIntError` conversion for free.
#[derive(Error, Debug)]
pub enum DoubleError {
    #[error("invalid first item to double")]
    InvalidFirst,
    #[error("unparsable number: {0}")]
    BadParse(#[from] std::num::ParseIntError),
}

fn double_first(vec: Vec<&str>) -> Result<i32, DoubleError> {
    let first = vec.first().ok_or(DoubleError::InvalidFirst)?;
    Ok(2 * first.parse::<i32>()?)
}

fn main() {
    for input in [vec!["42"], vec![], vec!["tofu"]] {
        match double_first(input) {
            Ok(n) => println!("The first doubled is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }
}
```
</details>

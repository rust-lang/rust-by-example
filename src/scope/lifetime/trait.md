# Traits

Annotation of lifetimes in trait methods basically are similar to functions.
Note that `impl` may have annotation of lifetimes too.

```rust,editable
// A struct with annotation of lifetimes.
#[derive(Debug)]
struct Borrowed<'a> {
    x: &'a i32,
}

// Annotate lifetimes to impl.
impl<'a> Default for Borrowed<'a> {
    fn default() -> Self {
        Self { x: &10 }
    }
}

fn main() {
    let b: Borrowed = Default::default();
    println!("b is {:?}", b);
}
```

### See also:

[`trait`s][trait]

[trait]: ../../trait.md

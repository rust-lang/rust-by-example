# Raw identifiers

Rust, like many programming languages, has the concept of "keywords". These
identifiers mean something to the language, and so you cannot use them in places
like variable names, function names, and other places. Raw identifiers let you
use keywords where they would not normally be allowed. This is particularly
useful when Rust introduces new keywords, and a library using an older edition
of Rust has a variable or function with the same name as a keyword introduced in
a newer edition.

For example, consider a crate `foo` compiled with the 2015 edition of Rust that
exports a function named `try`. This keyword is reserved for a new feature in
the 2018 edition, so without raw identifiers, we would have no way to name the
function.

```rust,ignore
extern crate foo;

fn main() {
    foo::try();
}
```

You'll get this error:

```text
error: expected identifier, found keyword `try`
 --> src/main.rs:4:4
  |
4 | foo::try();
  |      ^^^ expected identifier, found keyword
```

You can write this with a raw identifier:

```rust,ignore
extern crate foo;

fn main() {
    foo::r#try();
}
```

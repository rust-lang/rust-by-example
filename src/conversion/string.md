# To and from Strings

## `ToString`

To convert any type to a `String` it is as simple as implementing the [`ToString`]
trait for the type.

```rust,editable
use std::string::ToString;

struct Circle {
    radius: i32
}

impl ToString for Circle {
    fn to_string(&self) -> String {
        format!("Circle of radius {:?}", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
}
```

## Parsing a String

One of the more common types to convert a string into is a number. The idiomatic
approach to this is to use the [`parse`] function and provide the type for the
function to parse the string value into, this can be done either without type
inference or using the 'turbofish' syntax.

This will convert the string into the type specified so long as the [`FromStr`]
trait is implemented for that type. This is implemented for numerous types
within the standard library. To obtain this functionality on a user defined type
simply implement the [`FromStr`] trait for that type.

```rust
fn main() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!{"Sum: {:?}", sum};
}
```

[`ToString`]: https://doc.rust-lang.org/std/string/trait.ToString.html
[`parse`]: https://doc.rust-lang.org/std/primitive.str.html#method.parse
[`FromStr`]: https://doc.rust-lang.org/std/str/trait.FromStr.html

# To and from Strings

## Converting to String

To convert any type to a `String` is as simple as implementing the [`ToString`]
trait for the type. Rather than doing so directly, you should implement the
[`fmt::Display`][Display] trait which automagically provides [`ToString`] and
also allows printing the type as discussed in the section on [`print!`][print].

```rust,editable
use std::fmt;

struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
}
```

## Parsing a String

It's useful to convert strings into many types, but one of the more common
string operations is to convert them from string to number. The idiomatic
approach to this is to use the [`parse`] function and either to arrange for type
inference or to specify the type to parse using the 'turbofish' syntax. Both
alternatives are shown in the following example.

This will convert the string into the type specified as long as the [`FromStr`]
trait is implemented for that type. This is implemented for numerous types
within the standard library. To obtain this functionality on a user defined type
simply implement the [`FromStr`] trait for that type.

```rust,editable
fn main() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}
```

[`ToString`]: https://doc.rust-lang.org/std/string/trait.ToString.html
[Display]: https://doc.rust-lang.org/std/fmt/trait.Display.html
[print]: ../hello/print.md
[`parse`]: https://doc.rust-lang.org/std/primitive.str.html#method.parse
[`FromStr`]: https://doc.rust-lang.org/std/str/trait.FromStr.html

# Repeat

Macros can use `+` in the argument list to indicate that an argument may repeat
at least once, or `*`, to indicate that the argument may repeat zero or more
times.

In the following example, surrounding the matcher with `$(...),+` will match one
or more expression, separated by commas. Also note that the semicolon is
optional on the last case.

```rust,editable
// `find_min!` will calculate the minimum of any number of arguments.
macro_rules! find_min {
    // Base case:
    ($x:expr) => ($x);
    // `$x` followed by at least one `$y,`
    ($x:expr, $($y:expr),+) => (
        // Call `find_min!` on the tail `$y`
        std::cmp::min($x, find_min!($($y),+))
    )
}

fn main() {
    println!("{}", find_min!(1));
    println!("{}", find_min!(1 + 2, 2));
    println!("{}", find_min!(5, 2 * 3, 4));
}
```

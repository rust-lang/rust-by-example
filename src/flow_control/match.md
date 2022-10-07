# match

A match expression branches on a pattern. The exact form of matching that occurs depends on the pattern. A match expression has a scrutinee expression, which is the value to compare to the patterns. The scrutinee expression and the patterns must have the same type.

A match behaves differently depending on whether or not the scrutinee expression is a place expression or value expression. If the scrutinee expression is a value expression, it is first evaluated into a temporary location, and the resulting value is sequentially compared to the patterns in the arms until a match is found. The first arm with a matching pattern is chosen as the branch target of the match, any variables bound by the pattern are assigned to local variables in the arm's block, and control enters the block.

When the scrutinee expression is a place expression, the match does not allocate a temporary location; however, a by-value binding may copy or move from the memory location. When possible, it is preferable to match on place expressions, as the lifetime of these matches inherits the lifetime of the place expression rather than being restricted to the inside of the match.

An example of a match expression:

```rust,editable
let x = 1;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    4 => println!("four"),
    5 => println!("five"),
    _ => println!("something else"),
}
```
Another Example

```rust,editable
fn main() {
    let number = 13;
    // TODO ^ Try different values for `number`

    println!("Tell me about {}", number);
    match number {
        // Match a single value
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // TODO ^ Try adding 13 to the list of prime values
        // Match an inclusive range
        13..=19 => println!("A teen"),
        // Handle the rest of cases
        _ => println!("Ain't special"),
        // TODO ^ Try commenting out this catch-all arm
    }

    let boolean = true;
    // Match is an expression too
    let binary = match boolean {
        // The arms of a match must cover all the possible values
        false => 0,
        true => 1,
        // TODO ^ Try commenting out one of these arms
    };

    println!("{} -> {}", boolean, binary);
}
```

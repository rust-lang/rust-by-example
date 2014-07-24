Macros can be defined to handle indefinite arity (any number of arguments).
This can be accomplished using the *repetition* syntax and the `+`
and `*` operators. This is what the syntax looks like:

On the pattern side:

* `($($x:expr),+)`: matches a list (with at least one element) of `expr`s
  separated by commas. Examples: `('a', 'b')` and `(1 + 2, 3 * 4, 5 - 6)`

* `($($y:ident),*)`: matches a list of `ident`s separated by commas, but also
  handles the zero arity case. Examples: `()` and `(foo, bar)`

* `($($z:expr) separator +)`: in general, the `$(...)` is separated from the
`*` or `+` by whitespace and a separator. The whitespace is ignored. The
separator can be any text and many symbols.

* A trailing comma is handled separately from a repetition pattern. This
  works: `($($k:expr),+,)`

```rust
#![feature(macro_rules)]

macro_rules! echo {
    // Match a series of expressions separated by `,`; return the same
    // and convert the result into a string. `stringify!` causes
    // evaluation to be skipped.
    ($($x:expr),+) => { stringify!($($x),+) };
}

macro_rules! echo_space {
    // Same. Whitespace is ignored.
    ($($x:expr) , +) => { stringify!($($x) , +) };
}

macro_rules! echo_vert {
    // Split on `|`. `stringify` is important here.
    ($($x:expr)|+) => { stringify!($($x)|+) };
}

macro_rules! echo_dollar {
    // `$` isn't valid. It's a special macro key.
    ($($x:expr)$+) => { stringify!($($x)$+) };
}

macro_rules! echo_trail {
    // Split on `,` with trailing comma.
    ($($x:expr),+,) => { stringify!($($x),+,) };
}

macro_rules! echo_word {
    // Split on `word`. `stringify` is important here.
    ($($x:expr) word +) => { stringify!($($x) word +) };
}

macro_rules! echo_and {
    // Split on `and`. `stringify` is important here.
    ($($x:expr) and +) => { stringify!($($x) and +) };
}

/*// Block comment these 2 macros
macro_rules! echo_double_comma {
    // Error: only allows one symbol between `$(...)` and `+`
    ($($x:expr),,+) => { stringify!($($x),,+) };
}

macro_rules! echo_double_word {
    // Error: only allows one word between `$(...)` and `+`
    ($($x:expr) two words +) => { stringify!($($x) two words +) };
}
*/

fn main() {
    println!("{}", echo!(1u, 2u, 3u, 4u));
    // Error: Only a single symbol allowed
    //println!("{}", echo_double_comma!(1u,, 2u,, 3u,, 4u));
    println!("{}", echo_space!(1u, 2u, 3u, 4u));
    println!("{}", echo_vert!(1u | 2u | 3u));
    // Error: not valid. Don't split on `$`.
    //println!("{}", echo_dollar!(1u $ 2u $ 3u));
    println!("{}", echo_trail!(1u, 2u, 3u,));
    println!("{}", echo_word!(1u word 2u word 3u));
    println!("{}", echo_and!(1u and 2u and 3u));
    //Error: Only a single word allowed
    //println!("{}", echo_double_word!(1u two words 2u two words 3u));
}
```

On the expansion side:

* `[$($x),+]`: will expand the input arguments (the `$x`s) into the given
container. In this case, an array.
  Following the previous example: `['a', 'b']` and `[1 + 2, 3 * 4, 5 - 6]`
* `($($x),+)`: expand to a tuple: `('a', 'b')` and `(1 + 2, 3 * 4, 5 - 6)`
* `$($x),+`: expand to a sequence separated by `,` without a container.
* `$($x) and +`: expand to a sequence separated by `and` without a container.

```rust
#![feature(macro_rules)]

macro_rules! echo_and {
    // Replace `,` with `and`
    ($($x:expr),+) => { stringify!($($x) and +) };
}

macro_rules! echo_semicolon {
    // Replace `,` with `;`
    ($($x:expr),+) => { stringify!($($x);+) };
}

macro_rules! echo_vert {
    // Replace `,` with `|`
    ($($x:expr),+) => { stringify!($($x)|+) };
}

// However, remove `stringify!` and they will be evaluated
macro_rules! repeat {
    // Each `$x` will evaluate which will strip the `u`.
    ($($x:expr),+) => { ($($x),+) };
}

macro_rules! repeat_vert {
    // `,` split this into a sequence but `|` is bit-or
    ($($x:expr),+) => { ($($x) | +) };
}

macro_rules! repeat_and {
    // `and` makes this `(1 and 2 and 3 and 4)` which doesn't
    // evaluate so this errors (`and` isn't an operator).
    ($($x:expr),+) => { ($($x) and +) };
}

fn main() {
    // Evaluates to a number separated by `,`. Each number
    // is a single expression which is evaluated. Evaluation
    // strips the `u`.
    println!("{}", repeat!(1u, 2u, 3u, 4u));
    // bit-or evaluates `(1 | 2 | 3 | 4)` to `7`
    println!("{}", repeat_vert!(1u, 2u, 3u, 4u));
    // Error: `and` is not an operator
    //println!("{}", repeat_and!(1u, 2u, 3u, 4u));
}
```

For our example, we'll make a `min!` macro that can take any number of
arguments and will return the smallest one. Under the hood it will use the
`min` function that compares *two* arguments.

{repeat.play}

If you check the expansion you'll see this expression in the place of the last
macro:

``` rust
std::cmp::min(5u, std::cmp::min(2u * 3, 4u))
```

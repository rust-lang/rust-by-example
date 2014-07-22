#![feature(macro_rules)]

use std::iter::range_step;

// `macro_rules` behaves like a `match` expression
macro_rules! range (
    // The left side of the fat arrow is the `pattern` side, and the right side
    // is the `expansion` side
    ($end:expr) => {
        range(0, $end)
    };
    // The macro can have a different expansion for each arity
    ($start:expr, $end:expr) => {
        range($start, $end)
    };
    // ^ Each pattern-match arm must be terminated by a semicolon
    ($start:expr, $end:expr, $step:expr) => {
        range_step($start, $end, $step)
    };
    // The semicolon on the last arm is optional (is a trailing semicolon)
)

fn main() {
    for i in range!(10i) { print!("{}", i) }
    println!("");

    for i in range!(3i, 10) { print!("{}", i) }
    println!("");

    for i in range!(3i, 10, 2) { print!("{}", i) }
}


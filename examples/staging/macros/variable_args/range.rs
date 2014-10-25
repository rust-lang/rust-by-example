#![feature(macro_rules)]

use std::iter::range_step;

// range! will take up to 3 inputs and call range
// function depending on inputs
macro_rules! range (
    ($end:expr) => {
        range(0, $end)
    };
    ($start:expr, $end:expr) => {
        range($start, $end)
    };
    ($start:expr, $end:expr, $step:expr) => {
        range_step($start, $end, $step)
    };
)

fn main() {
    for i in range!(10i) { print!("{}", i) }
    println!("");

    for i in range!(3i, 10) { print!("{}", i) }
    println!("");

    for i in range!(3i, 10, 2) { print!("{}", i) }
}


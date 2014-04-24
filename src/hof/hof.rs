extern crate num;

// the AdditiveIterator trait adds the sum() method to iterators
use std::iter::{AdditiveIterator,count};
// the Integer trait adds the is_odd() method to integer primitives
use num::Integer;

fn main() {
    println!("Find the sum of all the squared odd numbers under 1000");
    let upper: int = 1000;

    // imperative approach
    // declare accumulator variable
    let mut acc = 0;
    // iterate 0, 1, 2, ... to infinity
    for n in count(0, 1) {
        // square the number
        let n_squared = n * n;

        if n_squared >= upper {
            // break loop if exceeded the upper limit
            break;
        } else if n_squared.is_odd() {
            // accumulate value, if it's odd
            acc += n_squared;
        }
    }
    println!("imperative style: {}", acc);

    // functional approach
    let sum_of_squared_odd_numbers =
        // all integers
        count(0, 1).
        // squared
        map(|n| n * n).
        // below upper limit
        take_while(|&n| n < upper).
        // that are odd
        filter(|n| n.is_odd()).
        // sum them
        sum();
    println!("functional style: {}", sum_of_squared_odd_numbers);
}

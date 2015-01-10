// The `AdditiveIterator` trait adds the `sum` method to iterators
use std::iter::AdditiveIterator;
use std::iter;

fn main() {
    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000u32;

    // Imperative approach
    // Declare accumulator variable
    let mut acc = 0;
    // Iterate: 0, 1, 2, ... to infinity
    for n in iter::count(0u32, 1) {
        // Square the number
        let n_squared = n * n;

        if n_squared >= upper {
            // Break loop if exceeded the upper limit
            break;
        } else if is_odd(n_squared) {
            // Accumulate value, if it's odd
            acc += n_squared;
        }
    }
    println!("imperative style: {}", acc);

    // Functional approach
    let sum_of_squared_odd_numbers =
        // All natural numbers
        iter::count(0u32, 1).
        // Squared
        map(|n| n * n).
        // Below upper limit
        take_while(|&n| n < upper).
        // That are odd
        filter(|n| is_odd(*n)).
        // Sum them
        sum();
    println!("functional style: {}", sum_of_squared_odd_numbers);
}

fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

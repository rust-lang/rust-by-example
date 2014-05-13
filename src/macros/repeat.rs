#![feature(macro_rules)]

// min! will calculate the minimum of any number of arguments
macro_rules! min {
    // base case
    ($x:expr) => {
        $x
    };
    // `$x` followed by at least one `$y,`
    ($x:expr, $($y:expr),+) => {
        // call min! on the tail `$y`
        std::cmp::min($x, min!($($y),+))
    }
}

fn main() {
    println!("{}", min!(1));
    println!("{}", min!(1 + 2 , 2));
    println!("{}", min!(5, 2 * 3, 4));
}

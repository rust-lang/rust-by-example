#![feature(macro_rules)]

// It's common to use recursion to handle indefinite arity
macro_rules! min {
    // base case
    ($x:expr) => {
        $x
    };
    // `$x` followed by at least another `expr`
    ($x:expr, $($y:expr),+) => {
        // Recursion! Call `min!` on the tail
        std::cmp::min($x, min!($($y),+))
    };
}

fn main() {
    println!("{}", min!(1u));
    println!("{}", min!(1u + 2 , 2u));
    println!("{}", min!(5u, 2u * 3, 4u));
}

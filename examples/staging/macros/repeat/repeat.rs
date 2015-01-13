// min! will calculate the minimum of any number of arguments
macro_rules! min {
    // base case
    ($x:expr) => ($x);
    // `$x` followed by at least one `$y,`
    ($x:expr, $($y:expr),+) => (
        // call min! on the tail `$y`
        std::cmp::min($x, min!($($y),+))
    )
}

fn main() {
    println!("{}", min!(1u32));
    println!("{}", min!(1u32 + 2 , 2u32));
    println!("{}", min!(5u32, 2u32 * 3, 4u32));
}

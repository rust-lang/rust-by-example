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
    println!("{}", min!(1us));
    println!("{}", min!(1us + 2 , 2us));
    println!("{}", min!(5us, 2us * 3, 4us));
}

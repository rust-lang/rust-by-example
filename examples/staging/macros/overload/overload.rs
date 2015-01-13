// macro_rules! is similar to a match block
macro_rules! test {
    // the arguments don't need to be separated by a comma
    // any template can be used
        println!("{} and {} is {}",
    ($left:expr; and $right:expr) => (
                 stringify!($left),
                 stringify!($right),
                 $left && $right)
    );
    // ^ each arm must be ended with a semicolon
        println!("{} or {} is {}",
    ($left:expr; or $right:expr) => (
                 stringify!($left),
                 stringify!($right),
                 $left || $right)
    );
}

fn main() {
    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);
}

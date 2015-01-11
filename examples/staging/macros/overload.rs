// macro_rules! is similar to a match block
macro_rules! test {
    // the arguments don't need to be separated by a comma
    // any template can be used
    ($left:expr; and $right:expr) => {
        println!("{} and {} is {}",
                 stringify!($left),
                 stringify!($right),
                 $left && $right)
    };
    // ^ each arm must be ended with a semicolon
    ($left:expr; or $right:expr) => {
        println!("{} or {} is {}",
                 stringify!($left),
                 stringify!($right),
                 $left || $right)
    };
}

fn main() {
    test!(1is + 1 == 2is; and 2is * 2 == 4is);
    test!(true; or false);
}

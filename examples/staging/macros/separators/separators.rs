#![feature(macro_rules)]

macro_rules! map {
    ($($key:expr => $value:expr),*) => ({
        let mut _temp = std::collections::HashMap::new();

        $(_temp.insert($key, $value);)*

        _temp
    });
    // Handle trailing commas
    ($($key:expr => $value:expr),+,) => (
        map!($($key => $value),+)
    );
}

fn main() {
    let alphabet = map! {
        'a' => "apple",
        'b' => "banana",
        'c' => "carrot",
    };

    println!("{}", alphabet);
}


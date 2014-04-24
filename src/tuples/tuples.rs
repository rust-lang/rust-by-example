// function that returns two values
fn chimpanzees() -> (int, &str) {
    (6, "Pan troglodytes")
}

fn elephants() -> (int, &str) {
    (9, "Elephas maximus")
}

fn penguins() -> (int, &str) {
    (4, "Spheniscus demersus")
}

fn wolves() -> (int, &str) {
    (6, "Canis lupus")
}

// tuples can be function arguments
fn show(pair: (int, &str)) {
    // destructuring a tuple
    let (amount, species) = pair;

    println!("There are {} {}", amount, species);
}

fn main() {
    // a tuple with a bunch of different types
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', "abc");

    // values can be extracted from the tuple using the valN methods
    println!("long tuple first value: {}", long_tuple.val0());
    println!("long tuple second value: {}", long_tuple.val1());

    // `let` can be used to bind a tuple members to variables
    let pair = (3, 4);
    let (x, y) = pair;
    println!("x is {}, and y is {}", x, y);

    // a tuple of tuples
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    println!("Animal inventory")
    show(chimpanzees());
    show(elephants());
    show(penguins());
    show(wolves());
}

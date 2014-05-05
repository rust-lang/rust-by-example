// tuples can be used as function arguments and as return values
fn reverse(pair: (int, bool)) -> (bool, int) {
    // `let` can be used to bind the members of a tuple to variables
    let (integer, boolean) = pair;

    (boolean, integer)
}

fn main() {
    // a tuple with a bunch of different types
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    // values can be extracted from the tuple using the valN methods
    println!("long tuple first value: {}", long_tuple.val0());
    println!("long tuple second value: {}", long_tuple.val1());

    // tuples can be tuple members
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // tuples are printable
    let pair = (1, true);
    println!("pair is {}", pair);

    println!("the reversed pair is {}", reverse(pair));

    // to create one element tuples, the comma is required to tell them
    // apart from a literal surrounded by parentheses
    println!("one element tuple: {}", (5,))
    println!("just an integer: {}", (5))
}

// Must derive `Debug` so `println!` can be used.
// `allow` required to silence warnings because only
// one variant is used.
#[allow(dead_code)]
#[derive(Debug)]
enum Color {
    // These 3 are specified solely by their name.
    Red,
    Blue,
    Green,
    // This requires 3 `i32`s and a name.
    RGB(i32, i32, i32),
}

fn main() {
    let color = Color::RGB(122, 17, 40);
    // TODO ^ Try different variants for `color`

    println!("What color is it?");
    // An `enum` can be destructured using a `match`.
    match color {
        Color::Red   => println!("The color is Red!"),
        Color::Blue  => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) => {
            println!("Red: {:?}, green: {:?}, and blue: {:?}!:", r, g, b);
        },
        // Don't need another arm because all variants have been examined
    }
}

// If you'd like to refer to the days without saying
// `Day::Monday`, then you use the following notation
// use Day::{
//     Monday,
//     Tuesday,
//     Wednesday,
//     Thursday,
//     Friday,
//     Saturday,
//     Sunday,
// }
//
// And you can refer to enum variants just like in C

// enum with implicit discriminator (starts at 0)
#[allow(dead_code)]
enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Day {
    fn mood(&self) {
        println!("{}", match *self {
            Day::Friday => "it's friday!",
            Day::Saturday | Day::Sunday => "weekend :-)",
            _ => "weekday...",
        })
    }
}

// enum with explicit discriminator
#[allow(dead_code)]
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    let today = Day::Monday;

    today.mood();

    // enums can be casted into integers
    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}

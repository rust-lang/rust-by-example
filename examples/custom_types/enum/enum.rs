// An attribute to hide warnings for unused code.
#![allow(dead_code)]

// Create an `enum` to classify someone. Note how both names
// and type information together specify the variant:
// `Skinny != Fat` and `Height(i32) != Weight(i32)`. Each
// is different and independent.
enum Person {
    // An `enum` may either be `unit-like`,
    Skinny,
    Fat,
    // like tuple structs,
    Height(i32),
    Weight(i32),
    // or like structures.
    Info { name: String, height: i32 }
}

// A function which takes a `Person` enum as an argument and
// returns nothing.
fn inspect(p: Person) {
    // Usage of an `enum` must cover all cases (irrefutable)
    // so a `match` is used to branch over it.
    match p {
        Person::Skinny    => println!("Is skinny!"),
        Person::Fat       => println!("Is fat!"),
        // Destructure `i` from inside the `enum`.
        Person::Height(i) => println!("Has a height of {}.", i),
        Person::Weight(i) => println!("Has a weight of {}.", i),
        // Destructure `Info` into `name` and `height`.
        Person::Info { name, height } => {
            println!("{} is {} tall!", name, height);
        },
    }
}

fn main() {
    let person = Person::Height(18);
    let danny  = Person::Weight(10);
    // `to_owned()` creates an owned `String` from a string slice.
    let dave   = Person::Info { name: "Dave".to_owned(), height: 72 };
    let john   = Person::Fat;
    let larry  = Person::Skinny;

    inspect(person);
    inspect(danny);
    inspect(dave);
    inspect(john);
    inspect(larry);
}

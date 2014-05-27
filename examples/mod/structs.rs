mod my {
    // a public struct with public fields
    pub struct WhiteBox<T> {
        pub contents: T,
    }

    // a public struct with private fields
    pub struct BlackBox<T> {
        contents: T,
    }

    impl<T> BlackBox<T> {
        // a public constructor
        pub fn new(contents: T) -> BlackBox<T> {
            BlackBox {
                contents: contents,
            }
        }
    }
}

fn main() {
    // public structs with public fields can be constructed as usual
    let white_box = my::WhiteBox { contents: "public information" };

    // and their fields can be normally accessed
    println!("The white box contains: {}", white_box.contents);

    // but public structs with private fields can't be constructed
    // Error: BlackBox has private fields
    //let black_box = my::BlackBox { contents: "classified information" };

    // however, structs with private fields can still be created using
    // constructors
    let black_box = my::BlackBox::new("classified information");

    // the private fields of a struct can't be accessed
    // Error: the contents field is private
    //println!("The black box contains: {}", black_box.contents);
}

mod my {
    // A public struct with public fields
    pub struct WhiteBox<T> {
        pub contents: T,
    }

    // A public struct with private fields
    #[allow(dead_code)]
    pub struct BlackBox<T> {
        contents: T,
    }

    impl<T> BlackBox<T> {
        // A public constructor
        pub fn new(contents: T) -> BlackBox<T> {
            BlackBox {
                contents: contents,
            }
        }
    }
}

fn main() {
    // Public structs with public fields can be constructed as usual
    let white_box = my::WhiteBox { contents: "public information" };

    // and their fields can be normally accessed
    println!("The white box contains: {}", white_box.contents);

    // but public structs with private fields can't be constructed
    // Error! `BlackBox` has private fields
    //let black_box = my::BlackBox { contents: "classified information" };
    // TODO ^ Try uncommenting this line

    // However, structs with private fields can still be created using
    // constructors
    let _black_box = my::BlackBox::new("classified information");

    // The private fields of a struct can't be accessed
    // Error! The `contents` field is private
    //println!("The black box contains: {}", _black_box.contents);
    // TODO ^ Try uncommenting this line
}

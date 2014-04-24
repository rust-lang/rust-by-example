// a unit struct
struct Foo;

// a unit struct that implements the Clone trait
#[deriving(Clone)]
struct Bar;

fn main() {
    // string is a reference to a global constant string
    let string: &'static str = "Hello World";

    // another_string is a copy of string, i.e. another reference to the
    // global constant
    let another_string = string;

    println!("{} and {}", string, another_string);

    // a heap allocated string
    let boxed_string = ~"Hello World";

    // this moves the ownership
    let new_boxed_string = boxed_string;

    // and the original boxed_string can't be used anymore
    //println!("{}", boxed_string);

    // this copies the original string into a new memory location
    let copied_boxed_string = new_boxed_string.clone();

    // original and copy can be used independently
    println!("{} and {}", new_boxed_string, copied_boxed_string);

    let foo = Foo;
    let bar = Bar;

    // Error: Foo is not cloneable
    //let copied_foo = Foo.clone();

    // Ok: bar implements the Clone trait
    let copied_bar = Bar.clone();
}

// a vanilla unit struct
struct Foo;

// a unit struct that implements the Clone trait
#[deriving(Clone)]
struct Dolly;

fn destroy_string(string: String) {
    // string gets destroyed in this scope
}

fn main() {
    // string is a reference to a global constant string
    let string: &'static str = "Hello World";

    // another_string is a copy of string, i.e. another reference to the
    // same global constant
    let another_string = string;

    // both can be used at the same time, there is no resource ownership
    println!("{} and {}", string, another_string);

    // a heap allocated string
    let boxed_string = String::from_str("Hello World");

    // this moves the ownership
    let new_boxed_string = boxed_string;

    // Error: the original boxed_string can't be used anymore
    //println!("{}", boxed_string);

    // this allocates memory and copies the string into it
    let copied_boxed_string = new_boxed_string.clone();

    // destroy the original string
    destroy_string(new_boxed_string);

    // copy can still be used
    println!("{}", copied_boxed_string);

    // instantiate unit structs
    let (foo, dolly) = (Foo, Dolly);

    // Error: Foo is not cloneable
    //let cloned_foo = Foo.clone();

    // Ok: bar implements the Clone trait
    let cloned_dolly = dolly.clone();
}

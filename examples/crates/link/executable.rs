// Link to `liberty`, import items under the `erty` module
extern crate erty;

fn main() {
    erty::public_function();

    // Error! `private_function` is private
    //erty::private_function();

    erty::indirect_access();
}

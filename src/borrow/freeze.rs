fn main() {
    let mut integer = 5;

    if true {
        // borrow integer
        let ref_to_integer = &integer;

        // Error: integer is frozen
        integer = 4;

        // ref_to_integer goes out of scope
    }

    // Ok: integer is not borrowed in this scope
    integer = 4;
}

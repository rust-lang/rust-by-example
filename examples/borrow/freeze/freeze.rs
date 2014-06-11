fn main() {
    let mut integer = 5;

    {
        // Borrow `integer`
        let ref_to_integer = &integer;

        // Error! `integer` is frozen in this scope
        integer = 4;
        // FIXME ^ Comment out this line

        // `ref_to_integer` goes out of scope
    }

    // Ok! `integer` is not frozen in this scope
    integer = 4;
}

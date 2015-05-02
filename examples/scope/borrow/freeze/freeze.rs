fn main() {
    let mut _integer = 5i32;

    {
        // Borrow `integer`
        let _ref_to_integer = &_integer;

        // Error! `integer` is frozen in this scope
        _integer = 4;
        // FIXME ^ Comment out this line

        // `ref_to_integer` goes out of scope
    }

    // Ok! `integer` is not frozen in this scope
    _integer = 4;
}

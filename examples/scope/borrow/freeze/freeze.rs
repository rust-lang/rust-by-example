fn main() {
    let mut mutable_integer = 7i32;

    {
        // Borrow `mutable_integer`
        let large_integer = &mutable_integer;

        // Error! `mutable_integer` is frozen in this scope
        mutable_integer = 50;
        // FIXME ^ Comment out this line

        // `large_integer` goes out of scope
    }

    // Ok! `mutable_integer` is not frozen in this scope
    mutable_integer = 3;
}

fn main() {
    let immutable_variable = 1;
    let mut mutable_variable = 1;

    // Ok
    mutable_variable += 1;

    // Error!
    immutable_variable += 1;
}

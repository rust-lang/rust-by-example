fn main() {
    // immutable variable
    let a_variable = 1;

    // mutable variable
    let mut mutable_variable = 2;

    // Error: attempted to modify an immutable variable
    a_variable += 1;

    // Ok
    mutable_variable += 1;
}

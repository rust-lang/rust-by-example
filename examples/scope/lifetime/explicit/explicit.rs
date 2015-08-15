// This example uses `main()` at the top so it can be read top to
// bottom. `main()` is usually at the bottom.
fn main() {
    // Create variables which will be borrowed.
    let (four, nine) = (4, 9);

    // Borrows (`&`) of both variables are passed into the function.
    // If a borrow is truly a borrow, the variable must be returned,
    // otherwise a borrow would be transferring ownership. This means
    // no matter what, *any* input which is borrowed via function or
    // otherwise, *must* still exist after the borrower ceases. In
    // other words, *Rule 2* must be true.
    print_refs(&four, &nine);

    // Note that there is no input even though the function specifies
    // a lifetime. The sizedness of that lifetime will be determined
    // by the caller (which is here) from available lifetimes (scopes).
    // Any lifetime chosen will then be larger than that of the function.
    failed_borrow();
}

// `print_refs` takes two references to `i32` which have different
// lifetimes `'a` and `'b`. These two lifetimes must both be at
// least as long as the function `print_refs`.
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

// There are no references to force `'a` to be larger than the
// function yet, `'a` remains larger than the function.
fn failed_borrow<'a>() {
    let _x = 12;

    // Attempting to use the lifetime `'a` as an explicit type
    // annotation inside the function will fail because the
    // lifetime of `&_x` is smaller than `y` has. A small lifetime
    // cannot be coerced into a larger one.
    //
    //let y: &'a i32 = &_x;
    // ERROR: `_x` does not live long enough
}

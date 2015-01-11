fn main() {
    // Type annotated variable
    let a_float: f64 = 1.0;

    // This variable is an `isize`
    let mut an_integer = 5is;

    // Error! The type of a variable can't be changed
    an_integer = true;
}

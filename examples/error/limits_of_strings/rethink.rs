use std::num::ParseIntError;
use std::fmt;

type Result<T> = std::result::Result<T, DoubleError>;

#[derive(Debug)]
// Define our error types. These may be customized however is useful for our error
// handling cases. Now we will be able to defer to the underlying tools error
// implementation, write our own errors, or something in between.
enum DoubleError {
    // We don't require any extra info to detail this error.
    EmptyVec,
    // We will defer to the parse error implementation for their error. Supplying extra
    // info would require adding more data to the type.
    Parse(ParseIntError),
}

// How the type is displayed is completely separate from where the errors are generated.
// We do not need to be concerned that the display style will clutter the complex logic
// our utility requires. They are separate matters which are handled separately.
//
// We don't store extra info about the errors. If we had desired, for example, to state
// which string failed to parse then we can't without modifying our types to carry that
// information accordingly.
impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DoubleError::EmptyVec =>
                write!(f, "please use a vector with at least one element"),
            // This is a wrapper so defer to the underlying types' own implementation
            // of `fmt`.
            DoubleError::Parse(ref e) => e.fmt(f),
        }
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
       // Change the error to our new type.
       .ok_or(DoubleError::EmptyVec)
       .and_then(|s| s.parse::<i32>()
            // Update to the new error type here also.
            .map_err(DoubleError::Parse)
            .map(|i| 2 * i))
}

fn print(result: Result<i32>) {
    match result {
        Ok(n)  => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let numbers = vec!["93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}

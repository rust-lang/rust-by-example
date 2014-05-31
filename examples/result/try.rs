use checked::{DivisionByZero,MathResult,NegativeLogarithm,NegativeSquareRoot};

mod checked;

// checked_op(x, y) = sqrt(ln(x / y))?
fn checked_op(x: f64, y: f64) -> MathResult {
    // if `checked_div` "fails", then DivisionByZero will be `return`ed
    let ratio = try!(checked::div(x, y));

    // if `checked_ln` "fails", then NegativeLogarithm will be `return`ed
    let ln = try!(checked::ln(ratio));

    checked::sqrt(ln)
}

fn main() {
    match checked_op(1.0, 10.0) {
        Err(why) => fail!(match why {
            NegativeLogarithm => "logarithm of negative number",
            DivisionByZero => "division by zero",
            NegativeSquareRoot => "square root of negative number",
        }),
        Ok(value) => println!("{}", value),
    }
}

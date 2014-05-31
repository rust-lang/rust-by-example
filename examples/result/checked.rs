// Mathematical "errors" we want to catch
#[deriving(Show)]
pub enum MathError {
    DivisionByZero,
    NegativeLogarithm,
    NegativeSquareRoot,
}

// type is used to alias the LHS to the RHS, usually to reduce typing/verbosity
pub type MathResult = Result<f64, MathError>;

pub fn div(x: f64, y: f64) -> MathResult {
    if y == 0.0 {
        // this operation would "fail", let's return the reason wrapped in Err
        Err(DivisionByZero)
    } else {
        // this operation is valid, return the result wrapped in Ok
        Ok(x / y)
    }
}

pub fn sqrt(x: f64) -> MathResult {
    if x < 0.0 {
        Err(NegativeSquareRoot)
    } else {
        Ok(x.sqrt())
    }
}

pub fn ln(x: f64) -> MathResult {
    if x < 0.0 {
        Err(NegativeLogarithm)
    } else {
        Ok(x.ln())
    }
}


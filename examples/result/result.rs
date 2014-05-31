mod checked;

// op(x, y) = sqrt(ln(x / y))
fn op(x: f64, y: f64) -> f64 {
    // Results can be matched, just like Options
    // This is a three level match pyramid!
    match checked::div(x, y) {
        Err(why) => fail!("{}", why),
        Ok(ratio) => match checked::ln(ratio) {
            Err(why) => fail!("{}", why),
            Ok(ln) => match checked::sqrt(ln) {
                Err(why) => fail!("{}", why),
                Ok(sqrt) => sqrt,
            },
        },
    }
}

fn main() {
    // will this fail?
    println!("{}", op(1.0, 10.0));
}

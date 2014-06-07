extern crate num;

use num::complex::{Complex,Complex32};

#[link(name = "m")]
extern {
    fn ccosf(z: Complex<f32>) -> Complex<f32>;
}

// safe wrapper
fn cos(z: Complex32) -> Complex32 {
    unsafe { ccosf(z) }
}

fn main() {
    // z = -1 + 0i
    let z = Complex::new(0.0, 1.0f32);

    println!("cos({}) = {}", z, cos(z));
}

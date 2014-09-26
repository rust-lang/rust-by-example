use complex::Complex32;
mod complex;

#[link(name = "m")]
extern {
    fn ccosf(z: Complex32) -> Complex32;
}

// safe wrapper
fn cos(z: Complex32) -> Complex32 {
    unsafe { ccosf(z) }
}

fn main() {
    // z = 0 + 1i
    let z = Complex32{re: 0.0, im: 1.0f32};

    println!("cos({}) = {}", z, cos(z));
}

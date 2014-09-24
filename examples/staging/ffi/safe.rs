mod complex;

#[link(name = "m")]
extern {
    fn ccosf(z: complex::Complex32) -> complex::Complex32;
}

// safe wrapper
fn cos(z: complex::Complex32) -> complex::Complex32 {
    unsafe { ccosf(z) }
}

fn main() {
    // z = -1 + 0i
    let z = complex::Complex32{re: 0.0, im: 1.0f32};

    println!("cos({}) = {}", z, cos(z));
}

use std::simd::f32x4;

fn main() {
    // create simd vectors
    let x = f32x4(1.0, 2.0, 3.0, 4.0);
    let y = f32x4(4.0, 3.0, 2.0, 1.0);

    // simd product
    let z = x * y;

    // like any struct, the simd vector can be destructured using `let`
    let f32x4(a, b, c, d) = z;

    println!("{:?}", (a, b, c, d));
}

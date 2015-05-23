#![feature(associated_consts)]

// 2 null structs.
#[allow(dead_code)]
struct Num;
#[allow(dead_code)]
struct Weird;

// Trait to define zero.
trait Zero {
    // `const` requires a type. Use a default of `0`.
    const ZERO: i32 = 0; 
}

// Trait to define one.
trait One {
    // Can use the type `Self` which defers to the `impl`.
    const ONE: Self;
}

// Use the default.
impl Zero for Num {}

// Change the default.
impl Zero for Weird {
    const ZERO: i32 = 9;
}

// Define one for `i32`.
impl One for i32 {
    // Type must agree with the `Self` type: `i32`.
    const ONE: i32 = 1;
}

fn main() {
    // These calls use the `UFCS` calling syntax.
    println!("ZERO for Num is {}", Num::ZERO);     // Default.
    println!("ZERO for Weird is {}", Weird::ZERO); // Customized.
    println!("ONE for i32 is {}", i32::ONE);       // Type specified in `impl`.
}

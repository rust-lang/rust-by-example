use std::ops::Add;
use std::marker::PhantomData;

/// Null enumerations define unit types.
#[derive(Debug, Clone, Copy)]
struct Inch;
#[derive(Debug, Clone, Copy)]
struct Mm;

/// `Length` is phantom type with hidden parameter `Unit`.
///
/// `f64` already implements the `Clone` and `Copy` traits.
#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64,PhantomData<Unit>);

/// The `Add` trait defines the behavior of the `+` operator.
impl<Unit> Add for Length<Unit> {
     type Output = Length<Unit>;

    // add() returns a new `Length` struct containing the sum.
    fn add(self, rhs: Length<Unit>) -> Length<Unit> {
        // `+` calls the `Add` implementation for `f64`.
        Length(self.0 + rhs.0, PhantomData)
    }
}

fn main() {
    // Specializes `one_foot` to have hidden parameter `Inch`.
    let one_foot:  Length<Inch> = Length(12.0, PhantomData);
    // `one_meter` has hidden parameter `Mm`.
    let one_meter: Length<Mm>   = Length(1000.0, PhantomData);

    // `+` calls the `add()` method we implemented for `Length<Unit>`.
    //
    // Since `Length` implements `Clone` + `Copy`, `add()` does not consume
    // `one_foot` and `one_meter` but makes a copy of them in `self` and `rhs`.
    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    // Addition works.
    println!("one foot + one_foot = {:?}", two_feet);
    println!("one meter + one_meter = {:?}", two_meters);

    // Nonsensical operations fail as they should:
    // Error: type mismatch.
    //let one_feter = one_foot + one_meter;
}


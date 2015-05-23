use std::ops::Add;
use std::marker::PhantomData;

/// Null enumerations to define unit types
#[derive(Debug, Clone, Copy)]
struct Inch;
#[derive(Debug, Clone, Copy)]
struct Mm;

/// Length is phantom type with hidden parameter `Unit`
#[derive(Debug, Clone, Copy)]
struct Length<Unit, T>(T,PhantomData<Unit>);

// This is similar to the header except `T` must also
// implement `Clone` and `Copy.
impl<Unit, T> Add<Length<Unit, T>> for Length<Unit, T> where
    T: Add<T, Output=T> + Clone + Copy {
    type Output = Length<Unit, T>;

    fn add(self, r: Length<Unit, T>) -> Length<Unit, T> {
        Length(self.0 + r.0, PhantomData)
    }
}

fn main() {
    // Specialize one_foot to have hidden parameter `Inch`
    let one_foot:  Length<Inch, f32> = Length(12.0, PhantomData);
    // one_meter has hidden parameter `Mm`
    let one_meter: Length<Mm, f32>   = Length(1000.0, PhantomData);

    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    // Addition works
    println!("one foot + one_foot = {:?}", two_feet);
    println!("one meter + one_meter = {:?}", two_meters);

    // Nonsensical operations fail as they should
    // Error: type mismatch
    //let one_feter = one_foot + one_meter;
}


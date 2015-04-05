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

/// impl X for Y {} means "implement the trait `X` for the Type `Y`"
/// The following lines implement the `Add` trait for Length.
///
/// The `<Unit, T: Add<T, Output=T>` after `impl` declares two generic
/// types, `Unit`, which can be any type, and `T`, which is a type that
/// must implement both traits `Copy` (which means no need to borrow,
/// move, or clone; you can just pass in a variable, and both the caller
/// and callee will own their own copy), and the trait `Add<T, Output=T>`.
///
/// `Add<T, Output=T>` means that the type implements Add, taking in a T
/// (meaning an i32 plus an i32, or an f64 plus an f64, etc.), and giving
/// back a T (i32 + i32 = i32).
///
/// So, this impl implements `Add<Length<Unit, T>` for `Length<Unit, T>`,
/// which means you can add a `Length` to another `Length` of the same type.
///
/// `type Output = Length<Unit, T>` means that this impl gives back a
/// `Length<Unit, T>`, so that
/// `Length<Mm, f64> + Length<Mm, f64> = Length<Mm, f64>`
impl<Unit, T: Add<T, Output=T> + Clone + Copy> Add<Length<Unit, T>>
        for Length<Unit, T> {
    type Output = Length<Unit, T>;

    fn add(self, r: Length<Unit, T>) -> Length<Unit, T> {
        let Length(ref left, _)  = self;
        let Length(ref right, _) = r;

        Length(*left + *right, PhantomData)
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


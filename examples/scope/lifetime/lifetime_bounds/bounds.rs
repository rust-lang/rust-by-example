use std::fmt::Debug; // Trait to bound with.

#[derive(Debug)]
// `Ref` contains a reference to type `T` where `T` is unknown
// with an unknown lifetime `'a`. `T` is bounded such that any
// references in `T` must outlive `'a`. In addition, the lifetime
// of `Ref` may not exceed `'a`.
struct Ref<'a, T: 'a>(&'a T);

// A generic function which prints using the `Debug` trait.
fn print<T>(t: T) where
    T: Debug {
    println!("`print`: t is {:?}", t);
}

// Here a reference to `T` is taken where `T` implements
// `Debug` and *all* references in `T` outlive `'a`. In
// addition, `'a` must outlive the function.
fn print_ref<'a, T>(t: &'a T) where
    T: Debug + 'a {
    println!("`print_ref`: t is {:?}", t);
}

fn main() {
    let x = 7;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
    print(ref_x);
}

// `F` must implement `Fn` for a function which takes no
// inputs and returns nothing. Exactly what is required
// for `diary`.
fn apply<F>(f: F) where
    F: Fn() {

    f()
}

fn main() {
    let x = 7;

    // Capture `x` into an anonymous type and implement
    // `Fn` for it. Store it in `print`.
    let print = || println!("{}", x);

    apply(print);
}

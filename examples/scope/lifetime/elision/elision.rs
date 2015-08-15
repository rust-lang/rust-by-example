// These two functions have essentially identical signatures
// because the compiler implicitly adds the lifetimes to
// the first.
fn elide_input(x: &i32) {
    println!("`elide_input`: {}", x)
}
fn annotated_input<'a>(x: &'a i32) {
    println!("`annotated_input`: {}", x)
}

// Similarly, lifetimes are added implicitly to the first.
fn elide_pass(x: &i32) -> &i32 { x }
fn annotated_pass<'a>(x: &'a i32) -> &'a i32 { x }

fn main() {
    let x = 3;
    
    elide_input(&x);
    annotated_input(&x);

    println!("`elide_pass`: {}", elide_pass(&x));
    println!("`annotated_pass`: {}", annotated_pass(&x));
}

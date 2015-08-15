// `x` is a reference with lifetime `'a` which is larger than `'b`.
// Both `'a` and `'b` are larger than `coerce_first`. Since `'a` is
// larger than `'b`, it may be coerced.
fn coerce_first<'a: 'b, 'b>(x: &'a i32, _: &'b i32) -> &'b i32 {
    x
}

fn main() {
    let x = 800;
    let y = 8;

    let borrow_big = &x;
    {
        // This reference is inside a scope and therefore is
        // smaller than the other borrow.
        let borrow_small = &y;

        let coerced = coerce_first(borrow_big, borrow_small);
        println!("`coerced` is {}", coerced);
    }
}

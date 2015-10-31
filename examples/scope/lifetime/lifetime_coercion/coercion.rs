// `x` is a reference with lifetime `'a` which is larger than `'b`.
fn coerce_first<'a: 'b, 'b>(x: &'a i32, _: &'b i32) -> &'b i32 {
    x // Since `'a` is larger, it may be coerced to match `'b`
}

fn main() {
    let x = 800;

    let borrow_big = &x;
    //let coerce_test;

    {
        let y = 8;
        // Since y is inside a scope, this reference has smaller
        // lifetime than borrow_big.
        let borrow_small = &y;

        let coerced = coerce_first(borrow_big, borrow_small);

        // Since the returned reference has a lifetime of y,
        //coerce_test = coerced; // this will not work

        println!("`coerced` is {}", coerced);
    }
}

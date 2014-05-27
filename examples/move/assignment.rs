use std::owned::Box;

fn main() {
    // stack allocated integer
    let x = 5;

    // copy `x` into `y`, there are no resources to move
    let y = x;

    // both values can be used, because they are independent
    println!("x is {}, and y is {}", x, y);

    // `a` is a pointer to a heap allocated integer
    let a = box 5;

    // copy `b` into `a`, now both point to the same heap allocated data
    // *but*, now b *owns* the heap allocated data
    // `b` is now in charge of freeing the memory in the heap
    let b = a;

    // Error: `a` can no longer access the data
    // because the resource has been *moved*
    println!("{} can not be used", a);

    // a goes out of scope
    // b goes out of scope, and the memory is freed
}

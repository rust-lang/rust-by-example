// This function takes ownership of the heap allocated memory
fn destroy_box(c: Box<int>) {
    println!("destroying a box that contains {}", c);

    // `c` will be destroyed in this scope, and the memory will be freed
}

fn main() {
    // Stack allocated integer
    let x = 5;

    // Copy `x` into `y`, there are no resources to move
    let y = x;

    // Both values can be independently used
    println!("x is {}, and y is {}", x, y);

    // `a` is a pointer to a heap allocated integer
    let a = box 5;

    println!("a contains: {}", a);

    // Copy `a` into `b`, now both are pointers to the same heap allocated
    // data, but now, `b` owns the heap allocated data
    // `b` is now in charge of freeing the memory in the heap
    let b = a;

    // Error! `a` can no longer access the data, because it no longer owns the
    // heap memory
    println!("a contains: {}", a);
    // FIXME ^ Comment out this line

    // Pass a copy of `b` to the function, and give up ownership
    destroy_box(b);

    // Error! For the same reason as the previous Error
    // Since the heap memory has been freed at this point, this action would
    // result in dereferencing freed memory
    println!("b contains: {}", b);
    // FIXME ^ Comment out this line
}

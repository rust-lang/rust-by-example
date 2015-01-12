#![feature(box_syntax)]

// This function takes ownership of the heap allocated memory
fn destroy_box(c: Box<i32>) {
    println!("destroying a box that contains {}", c);

    // `c` will be destroyed in this scope, and the memory will be freed
}

fn main() {
    // Stack allocated integer
    let x = 5u32;

    // "Copy" `x` into `y`, there are no resources to move
    let y = x;

    // Both values can be independently used
    println!("x is {}, and y is {}", x, y);

    // `a` is a pointer to a heap allocated integer
    let a = box 5i32;

    println!("a contains: {}", a);

    // "Move" `a` into `b`
    // Here's what happens under the hood: the pointer `a` gets copied (*not*
    // the data on the heap, just its address) into `b`. Now both are pointers
    // to the *same* heap allocated data. But now, `b` *owns* the heap
    // allocated data; `b` is now in charge of freeing the memory in the heap.
    let b = a;

    // After the previous move, `a` can no longer be used
    // Error! `a` can no longer access the data, because it no longer owns the
    // heap memory
    //println!("a contains: {}", a);
    // TODO ^ Try uncommenting this line

    // "Move" `b` into the function; `b` gives up ownership of the heap data
    destroy_box(b);

    // Since the heap memory has been freed at this point, this action would
    // result in dereferencing freed memory, but it's forbidden by the compiler
    // Error! Same reason as the previous Error
    //println!("b contains: {}", b);
    // TODO ^ Try uncommenting this line
}

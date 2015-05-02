fn create_box() {
    // Allocate an integer on the heap
    let _function_box = Box::new(3i32);

    // `_function_box` gets destroyed here, memory gets freed
}

fn main() {
    // Allocate an integer on the heap
    let _boxed_int = Box::new(5i32);

    // new (smaller) scope
    {
        // Another heap allocated integer
        let _short_lived_box = Box::new(4i32);

        // `_short_lived_box` gets destroyed here, memory gets freed
    }

    // Create lots of boxes
    for _ in 0u32..1_000 {
        create_box();
    }

    // `_boxed_int` gets destroyed here, memory gets freed
}

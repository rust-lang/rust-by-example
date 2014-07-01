fn create_box() {
    // Allocate an integer in the heap
    let _function_box = box 3i;

    // `_function_box` gets destroyed here, memory gets freed
}

fn main() {
    // Allocate an integer in the heap
    let _boxed_int = box 5i;

    // new (smaller) scope
    {
        // Another heap allocated integer
        let _short_lived_box = box 4i;

        // `_short_lived_box` gets destroyed here, memory gets freed
    }

    // Create lots of boxes
    for _ in range(0u, 1_000) {
        create_box();
    }

    // `_boxed_int` gets destroyed here, memory gets freed
}

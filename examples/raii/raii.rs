fn create_box() {
    let function_box = box 3;

    // function_box gets destroyed here, memory gets freed
}

fn main() {
    // allocate integer in the heap
    let boxed_int = box 5;

    if true {
        // new (smaller) block scope

        // another heap allocated integer
        let short_lived_box = box 4;

        // short_lived_box gets destroyed here, memory gets freed
    }

    // create lots of boxes
    for _ in range(0, 1_000) {
        create_box();
    }

    // boxed_int gets destroyed here, memory gets freed
}

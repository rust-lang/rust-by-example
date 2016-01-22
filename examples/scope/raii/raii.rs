// raii.rs
fn create_box() {
    // Allocate an integer on the heap
    let box1 = Box::new(3i32);

    // `box1` is destroyed here, and memory gets freed
}

fn main() {
    // Allocate an integer on the heap
    let box2 = Box::new(5i32);

    // A nested scope:
    {
        // Allocate an integer on the heap
        let box3 = Box::new(4i32);

        // `box3` is destroyed here, and memory gets freed
    }

    // Creating lots of boxes just for fun
    // There's no need to manually free memory!
    for _ in 0u32..1_000 {
        create_box();
    }

    // `box2` is destroyed here, and memory gets freed
}

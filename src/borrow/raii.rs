struct Homework;

// this function takes ownership of a Homework object
fn dog_eats_homework(homework: ~Homework) {
    // homework is destroyed in this scope, memory is freed from the heap
}

fn main() {
    // create homework
    let homework = ~Homework;

    // give up ownership of homework
    dog_eats_homework(homework);

    // Error: homework has been moved out of this scope
    //dog_eats_homework(homework);

    // blocks have their own scope
    if true {
        // allocate integer in the heap
        let boxed_int = ~5;

        // boxed_int goes out of scope and memory is freed from the heap
    }

    // Error: boxed_int doesn't exist in this scope
    //let another_boxed_int = boxed_int;
}

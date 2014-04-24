fn main() {
    // iterators can be collected into vectors
    let collected_iterator: Vec<int> = range(0, 10).collect();
    println!("collected range(0, 10) into: {}",
             collected_iterator.as_slice());

    // vec! can be used to initialize a vector
    let mut xs = vec![1, 2, 3];
    println!("vector: {}", xs.as_slice());

    // insert new element at the end of the vector
    println!("push 4 into the vector")
    xs.push(4);
    println!("vector: {}", xs.as_slice());

    // Error: immutable vectors can't be grow or shrink
    //collected_iterator.push(0);

    // the len method yields the current size of the vector
    println!("vector size: {}", xs.len());

    // indexing is done using the `get` method (indexing starts at 0)
    println!("second element: {}", xs.get(1));

    // remove last element from the vector
    println!("pop last element: {}", xs.pop());

    // out of bounds indexing yields a runtime failure
    println!("fourth element: {}", xs.get(3));
}

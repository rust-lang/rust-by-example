# Testcase: map-reduce

Rust makes it very easy to parallelise data processing, without many of the headaches traditionally associated with such an attempt.

The standard library provides great threading primitives out of the box.
These, combined with Rust's concept of Ownership and aliasing rules, automatically prevent
data races.

The aliasing rules (one writable reference XOR many readable references) automatically prevent
you from manipulating state that is visible to other threads. (Where synchronisation is needed,
there are synchronisation
primitives like `Mutex`es or `Channel`s.)

In this example, we will calculate the sum of all digits in a block of numbers.
We will do this by parcelling out chunks of the block into different threads. Each thread will sum
its tiny block of digits, and subsequently we will sum the intermediate sums produced by each
thread.

Note that, although we're passing references across thread boundaries, Rust understands that we're
only passing read-only references, and that thus no unsafety or data races can occur. Also because
the references we're passing have `'static` lifetimes, Rust understands that our data won't be
destroyed while these threads are still running. (When you need to share non-`static` data between
threads, you can use a smart pointer like `Arc` to keep the data alive and avoid non-`static`
lifetimes.)

```rust,editable
use std::thread;

// This is the `main` thread
fn main() {

    // This is our data to process.
    // We will calculate the sum of all digits via a threaded map-reduce algorithm.
    // Each whitespace separated chunk will be handled in a different thread.
    //
    // TODO: see what happens to the output if you insert spaces!
    let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668";

    // Make a vector to hold the child-threads which we will spawn.
    let mut children = vec![];

    /*************************************************************************
     * "Map" phase
     *
     * Divide our data into segments, and apply initial processing
     ************************************************************************/

    // split our data into segments for individual calculation
    // each chunk will be a reference (&str) into the actual data
    let chunked_data = data.split_whitespace();

    // Iterate over the data segments.
    // .enumerate() adds the current loop index to whatever is iterated
    // the resulting tuple "(index, element)" is then immediately
    // "destructured" into two variables, "i" and "data_segment" with a
    // "destructuring assignment"
    for (i, data_segment) in chunked_data.enumerate() {
        println!("data segment {} is \"{}\"", i, data_segment);

        // Process each data segment in a separate thread
        //
        // spawn() returns a handle to the new thread,
        // which we MUST keep to access the returned value
        //
        // 'move || -> u32' is syntax for a closure that:
        // * takes no arguments ('||')
        // * takes ownership of its captured variables ('move') and
        // * returns an unsigned 32-bit integer ('-> u32')
        //
        // Rust is smart enough to infer the '-> u32' from
        // the closure itself so we could have left that out.
        //
        // TODO: try removing the 'move' and see what happens
        children.push(thread::spawn(move || -> u32 {
            // Calculate the intermediate sum of this segment:
            let result = data_segment
                        // iterate over the characters of our segment..
                        .chars()
                        // .. convert text-characters to their number value..
                        .map(|c| c.to_digit(10).expect("should be a digit"))
                        // .. and sum the resulting iterator of numbers
                        .sum();

            // println! locks stdout, so no text-interleaving occurs
            println!("processed segment {}, result={}", i, result);

            // "return" not needed, because Rust is an "expression language", the
            // last evaluated expression in each block is automatically its value.
            result

        }));
    }


    /*************************************************************************
     * "Reduce" phase
     *
     * Collect our intermediate results, and combine them into a final result
     ************************************************************************/

    // combine each thread's intermediate results into a single final sum.
    //
    // we use the "turbofish" ::<> to provide sum() with a type hint.
    //
    // TODO: try without the turbofish, by instead explicitly
    // specifying the type of final_result
    let final_result = children.into_iter().map(|c| c.join().unwrap()).sum::<u32>();

    println!("Final sum result: {}", final_result);
}


```

### Assignments
It is not wise to let our number of threads depend on user inputted data.
What if the user decides to insert a lot of spaces? Do we _really_ want to spawn 2,000 threads?
Modify the program so that the data is always chunked into a limited number of chunks,
defined by a static constant at the beginning of the program.
#### Answer
You can find a solution for this assignment
[here][assignment_solution].

### See also:
* [Threads][thread]
* [vectors][vectors] and [iterators][iterators]
* [closures][closures], [move][move] semantics and [`move` closures][move_closure]
* [destructuring][destructuring] assignments
* [turbofish notation][turbofish] to help type inference
* [unwrap vs. expect][unwrap]
* [enumerate][enumerate]

[thread]: ../threads.md
[vectors]: ../../std/vec.md
[iterators]: ../../trait/iter.md
[destructuring]: https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#destructuring-to-break-apart-values
[closures]: ../../fn/closures.md
[move]: ../../scope/move.md
[move_closure]: https://doc.rust-lang.org/book/ch13-01-closures.html#closures-can-capture-their-environment
[turbofish]: https://doc.rust-lang.org/book/appendix-02-operators.html?highlight=turbofish
[unwrap]: ../../error/option_unwrap.md
[enumerate]: https://doc.rust-lang.org/book/loops.html#enumerate
[assignment_solution]: https://github.com/HosseinAssaran/Testcase-map-reduce-assignment.git

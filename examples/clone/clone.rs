// A unit struct without resources
#[deriving(Show)]
struct Nil;

// A tuple struct with resources that implements the `Clone` trait
#[deriving(Clone,Show)]
struct Pair(Box<int>, Box<int>);

fn main() {
    // Instantiate `Nil`
    let nil = Nil;
    // Copy `Nil`, there are no resources to move
    let copied_nil = nil;

    // Both `Nil`s can be used independently
    println!("original: {}", nil);
    println!("copy: {}", copied_nil);

    // Instantiate a `Pair`
    let pair = Pair(box 1, box 2);
    println!("original: {}", pair);

    // Copy `pair` into `moved_pair`, moves resources
    let moved_pair = pair;
    println!("copy: {}", moved_pair);

    // Error! `pair` has lost it resources
    //println!("original: {}", pair);
    // TODO ^ Try uncommenting this line

    // "Clone" `moved_pair` into `cloned_pair` (resources included)
    let cloned_pair = moved_pair.clone();

    // `Drop` the original pair
    drop(moved_pair);

    // Error! `moved_pair` has been `drop`ed
    //println!("copy: {}", moved_pair);
    // TODO ^ Try uncommenting this line

    // Clone can still be used
    println!("clone: {}", cloned_pair);
}

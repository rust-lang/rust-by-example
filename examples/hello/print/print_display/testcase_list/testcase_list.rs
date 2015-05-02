use std::fmt; // Import the `fmt` module.

// Define a structure named `List` containing a `Vec`.
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Dereference `self` and create a reference to `vec`
        // via destructuring.
        let List(ref vec) = *self;
        let len = vec.len(); // Save the vector length in `len`.

        // Iterate over `vec` in `v` while enumerating the iteration
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            // For every element except the last, format `write!`
            // with a comma. Use `try!` to return on errors.
            if count < len - 1 { try!(write!(f, "{}, ", v)) }
        }

        // `write!` the last value without special formatting.
        write!(f, "{}", vec[len-1])
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}

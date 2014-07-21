use std::mem;

struct Fibonacci {
    curr: uint,
    next: uint,
}

// Implement 'Iterator' for 'Fibonacci'
impl Iterator<uint> for Fibonacci {
    // The 'Iterator' trait only requires the 'next' method to be defined. The
    // return type is 'Option<T>', 'None' is returned when the 'Iterator' is
    // over, otherwise the next value is returned wrapped in 'Some'
    fn next(&mut self) -> Option<uint> {
        let new_next = self.curr + self.next;
        let new_curr = mem::replace(&mut self.next, new_next);

        // 'Some' is always returned, this is an infinite value generator
        Some(mem::replace(&mut self.curr, new_curr))
    }
}

// Returns a fibonacci sequence generator
fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 1, next: 1 }
}

fn main() {
    // Iterator that generates: 0, 1 and 2
    let mut sequence = range(0u, 3);

    println!("Four consecutive `next` calls on range(0, 3)")
    println!("> {}", sequence.next());
    println!("> {}", sequence.next());
    println!("> {}", sequence.next());
    println!("> {}", sequence.next());

    // The for construct will iterate an 'Iterator' until it returns 'None'.
    // Every 'Some' value is unwrapped and bound to a variable.
    println!("Iterate over range(0, 3) using for");
    for i in range(0u, 3) {
        println!("> {}", i);
    }

    // The 'take(n)' method will reduce an iterator to its first 'n' terms,
    // which is pretty useful for infinite value generators
    println!("The first four terms of the Fibonacci sequence are: ");
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }

    // The 'skip(n)' method will shorten an iterator by dropping its first 'n'
    // terms
    println!("The next four terms of the Fibonacci sequence are: ");
    for i in fibonacci().skip(4).take(4) {
        println!("> {}", i);
    }

    let array = [1u, 3, 3, 7];

    // The 'iter' method produces an 'Iterator' over an array/slice
    println!("Iterate the following array {}", array.as_slice());
    for i in array.iter() {
        println!("> {}", i);
    }
}

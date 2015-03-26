struct Fibonacci {
    curr: u32,
    next: u32,
}

// Implement 'Iterator' for 'Fibonacci'
impl Iterator for Fibonacci {
    type Item = u32;
    // The 'Iterator' trait only requires the 'next' method to be defined. The
    // return type is 'Option<T>', 'None' is returned when the 'Iterator' is
    // over, otherwise the next value is returned wrapped in 'Some'
    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        // 'Some' is always returned, this is an infinite value generator
        Some(self.curr)
    }
}

// Returns a fibonacci sequence generator
fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 1, next: 1 }
}

fn main() {
    // Iterator that generates: 0, 1 and 2
    let mut sequence = 0..3;

    println!("Four consecutive `next` calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    // The for construct will iterate an 'Iterator' until it returns 'None'.
    // Every 'Some' value is unwrapped and bound to a variable.
    println!("Iterate over 0..3 using for");
    for i in 0..3 {
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

    let array = [1u32, 3, 3, 7];

    // The 'iter' method produces an 'Iterator' over an array/slice
    println!("Iterate the following array {:?}", &array);
    for i in array.iter() {
        println!("> {}", i);
    }
}

use std::mem::replace;

struct Fibonacci {
    curr: uint,
    next: uint,
}

// implement Iterator for Fibonacci
impl Iterator<uint> for Fibonacci {
    // the Iterator trait only requires the next() method to be defined the
    // return value is Option<T>, None is returned when the Iterator is
    // over, otherwise the next value is returned wrapped in Some
    fn next(&mut self) -> Option<uint> {
        let new_next = self.curr + self.next;
        let new_curr = replace(&mut self.next, new_next);

        // Some is always returned, this is an infinite value generator
        Some(replace(&mut self.curr, new_curr))
    }
}

// returns a fibonacci sequence generator
fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 1, next: 1 }
}

fn main() {
    // iterator that generates: 0, 1 and 2
    let mut sequence = range(0, 3);

    println!("next in range(0, 3): {}", sequence.next());
    println!("next in range(0, 3): {}", sequence.next());
    println!("next in range(0, 3): {}", sequence.next());
    println!("next in range(0, 3): {}", sequence.next());

    // the for construct will iterate an Iterator until in returns None,
    // all the Some values are unwrapped and bind to a variable
    println!("iterate over range(0, 3) using for");
    for i in range(0, 3) {
        println!("{}", i);
    }

    // the take(n) method will reduce an iterator to its first n terms,
    // pretty useful for infinite value generators
    println!("The first ten terms of the fibonacci sequence are: ");
    for i in fibonacci().take(10) {
        println!("{}", i);
    }

    // the skip(n) method will shorten an iterator by dropping its first n
    // terms
    println!("The next ten terms of the fibonacci sequence are: ");
    for i in fibonacci().skip(10).take(10) {
        println!("{}", i);
    }

    let array = [1, 3, 3, 7];

    // the iter() method produces an iterator over an array/slice
    println!("iterate the following array {}", array.as_slice());
    for i in array.iter() {
        println!("{}", i);
    }
}

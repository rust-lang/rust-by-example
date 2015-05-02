#![feature(scoped)]

use std::thread;

static NTHREADS: i32 = 10;

// This is the `main` thread
fn main() {
    for i in 0..NTHREADS {
        // Spin up another thread
        let _ = thread::scoped(move || {
            println!("this is thread number {}", i)
        });
    }
}

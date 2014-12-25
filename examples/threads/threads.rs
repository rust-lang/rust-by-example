use std::thread::Thread;
use std::io::timer;
use std::time::duration::Duration;

static NTHREADS: int = 10;

// This is the `main` thread
fn main() {
    for i in range(0, NTHREADS) {
        // Spin up another thread
        Thread::spawn(move || {
            println!("this is thread number {}", i)
        }).detach();
    }

    // Wait for threads to complete
    let interval = Duration::milliseconds(1000);
    timer::sleep(interval);
}

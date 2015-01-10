use std::thread::Thread;

static NTHREADS: i32 = 10;

// This is the `main` thread
fn main() {
    for i in range(0, NTHREADS) {
        // Spin up another thread
        let _ = Thread::scoped(move || {
            println!("this is thread number {}", i)
        });
    }
}

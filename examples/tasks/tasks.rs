use std::thread::Thread;

static NTASKS: int = 10;

// This is the `main` task
fn main() {
    for i in range(0, NTASKS) {
        // Spin up another task
        Thread::spawn(move || {
            println!("this is task number {}", i)
        }).detach();
    }
}

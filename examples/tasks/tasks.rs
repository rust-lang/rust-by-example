static NTASKS: int = 10;

// This is the `main` task
fn main() {
    for i in range(0, NTASKS) {
        // Spin up another task
        spawn(proc() {
            println!("this is task number {}", i)
        });
    }
}

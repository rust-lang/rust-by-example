use std::thread;

fn main() {
    let interval = 1000;

    println!("Block for {} ms...", interval);
    thread::park_timeout_ms(interval);

    println!("Done");

    println!("Sleep for {} ms...", interval);
    thread::sleep_ms(interval);

    println!("Done");
}

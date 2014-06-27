use std::io::Timer;
use std::io::timer;
use std::iter;

static INTERVAL: u64 = 1000;

fn main() {
    // Create a timer object
    let mut timer = Timer::new().unwrap();

    // Create a one-shot notification
    // (superfluous type annotation)
    let oneshot: Receiver<()> = timer.oneshot(INTERVAL);

    println!("Wait {} ms...", INTERVAL);

    // Block the task until notification arrives
    oneshot.recv();

    println!("Done");

    println!("Sleep for {} ms...", INTERVAL);

    // This is equivalent to `timer.oneshot(INTERVAL).recv()`
    timer::sleep(INTERVAL);

    println!("Done");

    // The same timer can be used to generate periodic notifications
    // (superfluous type annotation)
    let metronome: Receiver<()> = timer.periodic(INTERVAL);

    println!("Countdown");
    for i in iter::range_step(5u, 0, -1) {
        // This loop will run once every second
        metronome.recv();

        println!("{}", i);
    }
    metronome.recv();
    println!("Ignition!");
}

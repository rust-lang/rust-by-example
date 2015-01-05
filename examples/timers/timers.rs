use std::io::Timer;
use std::io::timer;
use std::time::duration::Duration;
use std::iter;
use std::sync::mpsc::{Receiver};

fn main() {
    let interval = Duration::milliseconds(1000);
    // Create a timer object
    let mut timer = Timer::new().unwrap();

    // Create a one-shot notification
    // (superfluous type annotation)
    let oneshot: Receiver<()> = timer.oneshot(interval);

    println!("Wait {} ms...", interval.num_milliseconds());

    // Block the task until notification arrives
    oneshot.recv();

    println!("Done");

    println!("Sleep for {} ms...", interval.num_milliseconds());

    // This is equivalent to `timer.oneshot(interval).recv()`
    timer::sleep(interval);

    println!("Done");

    // The same timer can be used to generate periodic notifications
    // (superfluous type annotation)
    let metronome: Receiver<()> = timer.periodic(interval);

    println!("Countdown");
    for i in iter::range_step(5i, 0, -1) {
        // This loop will run once every second
        metronome.recv();

        println!("{}", i);
    }
    metronome.recv();
    println!("Ignition!");
}

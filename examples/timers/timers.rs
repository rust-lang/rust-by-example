#![feature(old_io)]
#![feature(std_misc)]
#![feature(step_by)]

use std::old_io::{timer, Timer};
use std::time::duration::Duration;
use std::sync::mpsc;

fn main() {
    let interval = Duration::milliseconds(1000);
    // Create a timer object
    let mut timer = Timer::new().unwrap();

    // Create a one-shot notification
    // (superfluous type annotation)
    let oneshot: mpsc::Receiver<()> = timer.oneshot(interval);

    println!("Wait {} ms...", interval.num_milliseconds());

    // Block the thread until notification arrives
    let _ = oneshot.recv();

    println!("Done");

    println!("Sleep for {} ms...", interval.num_milliseconds());

    // This is equivalent to `timer.oneshot(interval).recv()`
    timer::sleep(interval);

    println!("Done");

    // The same timer can be used to generate periodic notifications
    // (superfluous type annotation)
    let metronome: mpsc::Receiver<()> = timer.periodic(interval);

    println!("Countdown");
    for i in (5i32..0).step_by(-1) {
        // This loop will run once every second
        let _ = metronome.recv();

        println!("{}", i);
    }
    let _ = metronome.recv();
    println!("Ignition!");
}

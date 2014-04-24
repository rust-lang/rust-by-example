use std::io::timer::Timer;

fn main() {
    // create a timer object
    let mut timer = Timer::new().unwrap();

    // create a one shot notification (type annotation is superfluous)
    let oneshot: Receiver<()> = timer.oneshot(3000);

    println!("hold on...");

    // block the task until timeout
    oneshot.recv();

    // the same timer can be used to generate periodic notifications
    let metronome = timer.periodic(1000);

    let mut count = 0;
    println!("Start counting");
    loop {
        // loop will run once every second
        metronome.recv();

        count += 1;
        println!("{}", count);
    }
}

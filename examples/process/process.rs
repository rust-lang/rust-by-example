use std::io::process::{Command,ProcessOutput};

fn main() {
    // Initial command `rustc`
    let mut cmd = Command::new("rustc");
    // append the "--version" flag to the command
    cmd.arg("--version");

    // The `output` method will spawn `rustc --version`, wait until the process
    // finishes and return the output of the process
    match cmd.output() {
        Err(why) => panic!("couldn't spawn rustc: {}", why.desc),
        // Destructure `ProcessOutput`
        Ok(ProcessOutput { error: err, output: out, status: exit }) => {
            // Check if the process succeeded, i.e. the exit code was 0
            if exit.success() {
                // `out` has type `Vec<u8>`, convert it to a UTF-8 `$str`
                let s = String::from_utf8_lossy(out.as_slice());

                print!("rustc succeeded and stdout was:\n{}", s);
            } else {
                // `err` also has type `Vec<u8>`
                let s = String::from_utf8_lossy(err.as_slice());

                print!("rustc failed and stderr was:\n{}", s);
            }
        },
    }
}

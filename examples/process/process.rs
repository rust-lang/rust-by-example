use std::io::process::{Command,ProcessOutput};

fn main() {
    // initial command "rustc"
    let mut cmd = Command::new("rustc");
    // append the "--version" flag to the command
    cmd.arg("--version");

    // the `output` method will spawn `rustc --version`, wait until the process
    // finishes and return the output of the process
    match cmd.output() {
        Err(why) => fail!("couldn't spawn rustc: {}", why.desc),
        // destructure the ProcessOutput struct
        Ok(ProcessOutput { error: error, output: output, status: status }) => {
            // check if process succeeded, i.e. the exit code was 0
            if status.success() {
                // output has type `Vec<u8>`
                // convert to UTF-8 String (this operation could fail)
                let s = String::from_utf8(output).unwrap();

                print!("rustc succeeded and stdout was:\n{}", s);
            } else {
                // error also has type `Vec<u8>`
                let s = String::from_utf8(error).unwrap();

                print!("rustc succeeded and stderr was:\n{}", s);
            }
        },
    }
}

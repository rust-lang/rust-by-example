use std::old_io::process::Command;

static PANGRAM: &'static str =
"the quick brown fox jumped over the lazy dog\n";

fn main() {
    // Spawn the `wc` command
    let mut process = match Command::new("wc").spawn() {
        Err(why) => panic!("couldn't spawn wc: {}", why.desc),
        Ok(process) => process,
    };

    {
        // The `stdin` field has type `Option<PipeStream>`
        // `take` will take the value out of an `Option`, leaving `None` in
        // its place.
        //
        // Note that we take ownership of `stdin` here
        let mut stdin = process.stdin.take().unwrap();

        // Write a string to the stdin of `wc`
        match stdin.write_str(PANGRAM) {
            Err(why) => panic!("couldn't write to wc stdin: {}", why.desc),
            Ok(_) => println!("sent pangram to wc"),
        }

        // `stdin` gets `drop`ed here, and the pipe is closed
        // This is very important, otherwise `wc` wouldn't start processing the
        // input we just sent
    }

    // The `stdout` field also has type `Option<PipeStream>`
    // the `as_mut` method will return a mutable reference to the value
    // wrapped in a `Some` variant
    match process.stdout.as_mut().unwrap().read_to_string() {
        Err(why) => panic!("couldn't read wc stdout: {}", why.desc),
        Ok(string) => print!("wc responded with:\n{}", string),
    }
}

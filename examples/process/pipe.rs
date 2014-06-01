use std::io::process::Command;

static PANGRAM: &'static str = "the quick brown fox jumped over the lazy dog";

fn main() {
    // spawn the `wc` command
    let mut process = match Command::new("wc").spawn() {
        Err(why) => fail!("couldn't spawn wc: {}", why.desc),
        Ok(process) => process,
    };

    if true {
        // the `stdin` field has type Option<PipeStream>
        // `take_unwrap` will take the value wrapped in a Some variant
        // note that we take ownership of stdin here
        let mut stdin = process.stdin.take_unwrap();

        // write a string to the stdin of wc
        match stdin.write_str(PANGRAM) {
            Err(why) => fail!("couldn't write to wc stdin: {}", why.desc),
            Ok(_) => println!("sent pangram to wc"),
        }

        // stdin goes out of scope here, and the pipe is closed
        // this is very important, otherwise wc wouldn't start processing the
        // input we just sent
    }

    // the `stdout` field also has type Option<PipeStream>
    // the `get_mut_ref` method will return a mutable reference to the pipe
    match process.stdout.get_mut_ref().read_to_str() {
        Err(why) => fail!("couldn't read wc stdout: {}", why.desc),
        Ok(string) => print!("wc responded with:\n{}", string),
    }
}

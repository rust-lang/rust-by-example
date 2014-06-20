use std::os;

fn increase(number: int) {
    println!("{}", number + 1);
}

fn decrease(number: int) {
    println!("{}", number - 1);
}

fn help() {
    println!("usage:
match_args <integer>
match_args {{increase|decrease}} <integer>");
}

fn main() {
    let args = os::args();

    match args.as_slice() {
        // no arguments passed
        [ref name] => {
            println!("My name is '{}'. Try passing some arguments!", name);
        },
        // one argument passed
        [ref _name, ref num] => {
            if num.as_slice() == "42" {
                println!("This is the answer!");
            } else {
                println!("This is not the answer.");
            }
        },
        // one command and one argument passed
        [ref _name, ref cmd, ref num] => {
            let mut number: int;
            // parse the number
            match from_str(num.as_slice()) {
                Some(n) => {
                    number = n;
                },
                None => {
                    println!("error: second argument not an integer");
                    help();
                    return;
                },
            }
            // parse the command
            match cmd.as_slice() {
                "increase" => increase(number),
                "decrease" => decrease(number),
                _ => {
                    println!("error: invalid command");
                    help()
                },
            }
        },
        // all the other cases
        _ => {
            // show a help message
            help();
        }
    }
}

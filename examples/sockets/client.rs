#![feature(old_io)]
#![feature(old_path)]
#![feature(env)]
#![feature(core)]

use std::env;
use common::SOCKET_PATH;
use std::old_io::net::pipe::UnixStream;

mod common;

fn main() {
    // `args` returns the arguments passed to the program
    let args: Vec<String> = env::args().map(|x| x.to_string())
                                       .collect();
    let socket = Path::new(SOCKET_PATH);

    // First argument is the message to be sent
    let message = match args.as_slice() {
        [_, ref message] => message.as_slice(),
        _ => panic!("wrong number of arguments"),
    };

    // Connect to socket
    let mut stream = match UnixStream::connect(&socket) {
        Err(_) => panic!("server is not running"),
        Ok(stream) => stream,
    };

    // Send message
    match stream.write_str(message) {
        Err(_) => panic!("couldn't send message"),
        Ok(_) => {}
    }
}

use common::SOCKET_PATH;
use std::io::net::unix::UnixStream;
use std::os;

mod common;

fn main() {
    // args: is an array of the arguments passed to the program
    let args = os::args();
    let socket = Path::new(SOCKET_PATH);

    // First argument is the message to be sent
    let message = match args.as_slice() {
        [_, ref message] => message.as_slice(),
        _ => fail!("wrong number of arguments"),
    };

    // Connect to socket
    let mut stream = match UnixStream::connect(&socket) {
        Err(_) => fail!("server is not running"),
        Ok(stream) => stream,
    };

    // Send message
    match stream.write_str(message) {
        Err(_) => fail!("couldn't send message"),
        Ok(_) => {}
    }
}

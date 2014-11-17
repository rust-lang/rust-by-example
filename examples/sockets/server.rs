use common::SOCKET_PATH;
use std::io::fs;
use std::io::fs::PathExtensions;
use std::io::net::pipe::UnixListener;
use std::io::{Acceptor,Listener};

mod common;

fn main() {
    let socket = Path::new(SOCKET_PATH);

    // Delete old socket if necessary
    if socket.exists() {
        fs::unlink(&socket).unwrap();
    }

    // Bind to socket
    let stream = match UnixListener::bind(&socket) {
        Err(_) => panic!("failed to bind socket"),
        Ok(stream) => stream,
    };

    println!("Server started, waiting for clients");

    // Iterate over clients, blocks if no client available
    for mut client in stream.listen().incoming() {
        println!("Client said: {}", client.read_to_string().unwrap());
    }
}

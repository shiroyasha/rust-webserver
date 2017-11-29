use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let address = "0.0.0.0:8080";
    let listener = TcpListener::bind(address).unwrap();

    println!("Listening on {}", address);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);

    }
}

fn handle_connection(mut stream : TcpStream) {
    println!("Connection Established!");

    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    println!("Result: {}", String::from_utf8_lossy(&buffer[..]));
}

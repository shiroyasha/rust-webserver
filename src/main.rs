use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;

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

    let mut buffer = [0; 2048];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut content = String::new();

    file.read_to_string(&mut content).unwrap();

    let response = format!("{}{}", status_line, content);

    println!("Sending Response: {}", response);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

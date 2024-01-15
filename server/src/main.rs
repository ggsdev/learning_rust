use std::io::Read;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let addr: &str = "127.0.0.1:7878";

    let listener: TcpListener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();

        handle_connection(stream);

        println!("Connected");
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]))
}

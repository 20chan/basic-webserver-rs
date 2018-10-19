use std::io::prelude::*;
use std::thread;
use std::time::Duration;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        thread::spawn(|| {
            handle_connection(stream.unwrap());
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status, html) = if buffer.starts_with(get) {
        ("200 OK", "<html><body>Oh it's working!</body></html>")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(3));
        ("200 OK", "<html><body>Slept well :)</body></html>")
    } else {
        ("400 NOT FOUND", r"<html><body>404 Not Found ¯\_(ツ)_/¯</body></html>")
    };

    let response = format!("HTTP/1.1 {}\r\n\r\n{}", status, html);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
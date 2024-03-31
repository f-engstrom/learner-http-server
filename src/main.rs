// Uncomment this block to pass the first stage
use std::{
    io::{self, BufRead, Write},
    net::TcpListener,
};

fn handle_connection(stream: &std::net::TcpStream) -> Vec<u8> {
    let mut reader = io::BufReader::new(stream);
    let received: Vec<u8> = reader.fill_buf().unwrap().to_vec();
    reader.consume(received.len());

    let msg = String::from_utf8(received).unwrap();
    let path = msg.lines().nth(0).unwrap();
    match path {
        path if path.contains("GET") => {
            println!("OK");
            let route = path
                .split_whitespace()
                .nth(1)
                .unwrap()
                .split("/")
                .last()
                .unwrap();
            let length = route.len();
            println!("hmm {:?}", route);
            let content = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {length}\r\n\r\n{route}").into_bytes();
            content
        }
        error => {
            println!("404 {}", error);
            b"HTTP/1.1 404 Not Found\r\n\r\n".to_vec()
        }
    }
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    //
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                let msg = handle_connection(&stream);
                stream.write(&msg);
                // Ok(())
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

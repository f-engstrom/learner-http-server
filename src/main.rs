// Uncomment this block to pass the first stage
use std::{
    io::{self, BufRead, Write},
    net::TcpListener,
};

fn handle_connection(stream: &std::net::TcpStream) -> &'static [u8] {
    let mut reader = io::BufReader::new(stream);
    let received: Vec<u8> = reader.fill_buf().unwrap().to_vec();
    reader.consume(received.len());

    let msg = String::from_utf8(received).unwrap();
    let path = msg.lines().nth(0).unwrap();
    match path {
        "GET / HTTP/1.1" => {
            println!("OK");
            b"HTTP/1.1 200 OK\r\n\r\n"
        }
        error => {
            println!("404 {}", error);
            b"HTTP/1.1 404 Not Found\r\n\r\n"
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
                stream.write(msg);
                // Ok(())
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

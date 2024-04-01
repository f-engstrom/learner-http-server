// Uncomment this block to pass the first stage
use std::{
    io::{self, BufRead, Write},
    net::TcpListener,
};

use itertools::Itertools;

struct Request {
    method: String,
    path: String,
    http_version: String,
    headers: Vec<String>,
    body: Option<String>,
}

fn parse_request(request: String) -> Request {
    let mut request_lines = request.lines();
    let request_line = request_lines.next().unwrap();
    let (method, path, http_version) = request_line.split_whitespace().collect_tuple().unwrap();
    let headers = [String::from("hej")].to_vec();
    let req = Request {
        method: method.to_owned(),
        path: path.to_owned(),
        http_version: http_version.to_owned(),
        headers,
        body: None,
    };
    req
}

fn handle_connection(stream: &std::net::TcpStream) -> Vec<u8> {
    let mut reader = io::BufReader::new(stream);
    let received: Vec<u8> = reader.fill_buf().unwrap().to_vec();
    reader.consume(received.len());

    let msg = String::from_utf8(received).unwrap();
    let req = parse_request(msg);

    println!("hmm {:?}", req.path);

    match req.path {
        path if path.contains("/echo/") => {
            let echo_route = path.strip_prefix("/echo/").unwrap();
            let length = echo_route.len();
            println!("hmm {:?}", echo_route);
            let content = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {length}\r\n\r\n{echo_route}").into_bytes();
            content
        }
        path if path == "/" => b"HTTP/1.1 200 OK\r\n\r\n".to_vec(),
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

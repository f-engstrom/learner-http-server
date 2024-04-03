// Uncomment this block to pass the first stage
use std::{
    collections::HashMap,
    env, fs,
    io::{self, BufRead, Write},
    net::TcpListener,
    thread,
};

use itertools::Itertools;

struct Request {
    method: String,
    path: String,
    http_version: String,
    headers: HashMap<String, String>,
    body: Option<String>,
}

fn parse_request(request: String) -> Request {
    let mut request_lines = request.lines();
    let request_line = request_lines.next().unwrap();
    let (method, path, http_version) = request_line.split_whitespace().collect_tuple().unwrap();
    let mut headers = HashMap::new();
    for line in request_lines {
        let header_value = line.split_once(" ");
        if header_value.is_some() {
            let (header, value) = header_value.unwrap();
            headers.insert(header.to_owned(), value.to_owned());
        }
    }
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
        path if path.contains("/user-agent") => {
            println!("user {}", path);
            let user_agent = req.headers.get("User-Agent:").unwrap();
            let length = user_agent.len();

            let content = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {length}\r\n\r\n{user_agent}").into_bytes();
            content
        }
        path if path.contains("/files") => {
            let mut dir: String = String::new();
            let mut args = env::args().skip(1);
            while let Some(arg) = args.next() {
                match &arg[..] {
                    "--directory" => {
                        if let Some(arg_directory) = args.next() {
                            dir = arg_directory;
                        } else {
                            panic!("No value specified for parameter --directory.");
                        }
                    }
                    _ => {
                        if arg.starts_with('-') {
                            println!("Unkown argument {}", arg);
                        } else {
                            println!("Unkown positional argument {}", arg);
                        }
                    }
                }
            }
            let requested_file = path.strip_prefix("/files/").unwrap();
            let file_path = format!("{dir}/{requested_file}");
            println!("files {}", requested_file);
            let mut content = b"HTTP/1.1 404 Not Found\r\n\r\n".to_vec();

            match fs::read(file_path) {
                Ok(file) => {
                    let file_bytes = String::from_utf8_lossy(&file);
                    let length = file_bytes.len();
                    println!("{:?}", file_bytes);
                    content = format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {length}\r\n\r\n{file_bytes}").into_bytes();
                }
                Err(_) => {}
            };

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
                thread::spawn(move || {
                    let msg = handle_connection(&stream);
                    stream.write(&msg);
                });

                // Ok(())
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

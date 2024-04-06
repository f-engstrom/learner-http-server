use crate::models::Request;
use std::{env, fs, str::Matches};

pub static ECHO_ROUTE: &str = "echo";
pub fn echo_route_handler(req: &Request) -> Vec<u8> {
    println!("echo route");
    let echo_route = req.path_name.strip_prefix("/echo/").unwrap();
    let length = echo_route.len();
    println!("hmm {:?}", echo_route);
    let content = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {length}\r\n\r\n{echo_route}").into_bytes();
    content
}

pub static USER_AGENT_ROUTE: &str = "/user-agent";
pub fn user_agent_route_handler(req: &Request) -> Vec<u8> {
    println!("user agent route");
    let user_agent = req.headers.get("User-Agent:").unwrap();
    let length = user_agent.len();

    let content = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {length}\r\n\r\n{user_agent}").into_bytes();
    content
}
pub static FILES_ROUTE: &str = "files";
pub fn files_route_handler(req: &Request) -> Vec<u8> {
    println!("files route");
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
    let requested_file = req.path_name.strip_prefix("/files/").unwrap();
    let file_path = format!("{dir}/{requested_file}");
    match &req.method {
        method if method.contains("GET") => {
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
        method if method.contains("POST") => {
            println!("files post {:?}", req.body);
            let file = req.body.clone().unwrap();

            fs::write(file_path, file);
            b"HTTP/1.1 201 OK\r\n\r\n".to_vec()
        }
        _ => b"HTTP/1.1 404 Not Found\r\n\r\n".to_vec(),
    }
}

pub static ROOT_ROUTE: &str = "/";
pub fn root_route_handler(req: &Request) -> Vec<u8> {
    println!("root route");

    b"HTTP/1.1 200 OK\r\n\r\n".to_vec()
}

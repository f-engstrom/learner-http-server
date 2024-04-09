use crate::{models, request::build_response};
use models::*;
use std::{env, fs};

pub static ECHO_ROUTE: &str = "echo";
pub fn echo_route_handler(req: &Request) -> Vec<u8> {
    println!("echo route");
    let echo_route = req.path_name.strip_prefix("/echo/").unwrap();
    let length = echo_route.len();
    println!("hmm {:?}", echo_route);

    build_response(
        ResponseCode::Ok,
        Some(Vec::from([
            ResponseHeaders::ContentType.format_header_value("text/plain"),
            ResponseHeaders::ContentLength.format_header_value(&length.to_string()),
        ])),
        Some(echo_route.as_bytes().to_vec()),
    )
}

pub static USER_AGENT_ROUTE: &str = "/user-agent";
pub fn user_agent_route_handler(req: &Request) -> Vec<u8> {
    println!("user agent route");
    let user_agent = req.headers.get("User-Agent:").unwrap().to_owned();
    let length = user_agent.len();

    build_response(
        ResponseCode::Ok,
        Some(Vec::from([
            ResponseHeaders::ContentType.format_header_value("text/plain"),
            ResponseHeaders::ContentLength.format_header_value(&length.to_string()),
        ])),
        Some(user_agent.into_bytes()),
    )
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
            match fs::read(file_path) {
                Ok(file) => build_response(
                    ResponseCode::Ok,
                    Some(Vec::from([
                        ResponseHeaders::ContentType
                            .format_header_value("application/octet-stream"),
                        ResponseHeaders::ContentLength.format_header_value(&file.len().to_string()),
                    ])),
                    Some(file),
                ),
                Err(_) => build_response(ResponseCode::NotFound, None, None),
            }
        }
        method if method.contains("POST") => {
            println!("files post {:?}", req.body);
            let file = req.body.clone().unwrap();

            match fs::write(file_path, file) {
                Ok(_) => build_response(ResponseCode::Created, None, None),
                _ => build_response(ResponseCode::InternalServerError, None, None),
            }
        }
        _ => build_response(ResponseCode::NotAllowed, None, None),
    }
}

pub static ROOT_ROUTE: &str = "/";
pub fn root_route_handler(req: &Request) -> Vec<u8> {
    println!("root route");
    let mut dir: String = String::from("testfiler");
    let mut requested_file: String = String::from("index.html");
    let file_path = format!("{dir}/{requested_file}");

    match fs::read(file_path) {
        Ok(file) => build_response(
            ResponseCode::Ok,
            Some(Vec::from([
                ResponseHeaders::ContentType.format_header_value("text/html; charset=UTF-8"),
                ResponseHeaders::ContentLength.format_header_value(&file.len().to_string()),
            ])),
            Some(file),
        ),
        Err(_) => build_response(ResponseCode::NotFound, None, None),
    }
}

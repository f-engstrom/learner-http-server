use crate::{request, route};

use route::{
    echo_route_handler, files_route_handler, root_route_handler, user_agent_route_handler,
    ECHO_ROUTE, FILES_ROUTE, ROOT_ROUTE, USER_AGENT_ROUTE,
};
use std::{
    env, fs,
    io::{BufRead, BufReader},
};

pub fn handle_connection(stream: &std::net::TcpStream) -> Vec<u8> {
    let mut reader = BufReader::new(stream);
    let received: Vec<u8> = reader.fill_buf().unwrap().to_vec();
    reader.consume(received.len());

    let msg = String::from_utf8(received).unwrap();
    let req = request::parse_request(msg);

    println!("handle connection path name {:?}", req.path_name);

    match &req.path_name {
        path if path.contains(ECHO_ROUTE) => echo_route_handler(&req),
        path if path.contains(USER_AGENT_ROUTE) => user_agent_route_handler(&req),
        path if path.contains(FILES_ROUTE) => files_route_handler(&req),
        path if path.contains(ROOT_ROUTE) => root_route_handler(&req),
        error => {
            println!("404 {}", error);
            b"HTTP/1.1 404 Not Found\r\n\r\n".to_vec()
        }
    }
}

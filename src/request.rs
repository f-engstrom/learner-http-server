use crate::models;
use itertools::Itertools;
use models::*;

use std::collections::HashMap;

pub fn parse_request(request: String) -> Request {
    let mut request_lines = request.lines();
    let request_line = request_lines.next().unwrap();
    let (method, path_name, http_version) =
        request_line.split_whitespace().collect_tuple().unwrap();
    let mut headers = HashMap::new();
    let mut body: Option<String> = None;
    //om tom line så börjar bodyn köra nån next() och mappa? som cmd line?
    // for line in request_lines {
    //     let header_value = line.split_once(" ");
    //     if header_value.is_some() {
    //         let (header, value) = header_value.unwrap();
    //         //if unknown header skip and log
    //         headers.insert(header.to_owned(), value.to_owned());
    //     } else {
    //         break;
    //     }
    // }
    while let Some(line) = request_lines.next() {
        if line.is_empty() {
            body = match request_lines.next() {
                Some(line) => Some(String::from(line)),
                _ => None,
            };
            break;
        }
        let header_value = line.split_once(" ");
        if header_value.is_some() {
            let (header, value) = header_value.unwrap();
            //if unknown header skip and log
            headers.insert(header.to_owned(), value.to_owned());
        }
    }
    println!("{:?}", request_lines);
    println!("{:?}", headers);

    let req = Request {
        method: method.to_owned(),
        path_name: path_name.to_owned(),
        http_version: http_version.to_owned(),
        headers,
        body,
    };
    req
}
pub fn build_response(
    response_type: String,
    headers: HashMap<String, String>,
    body: Option<&str>,
) -> Vec<u8> {
    format!("HTTP/1.1 200 OK\r\n\r\n").into_bytes()
}

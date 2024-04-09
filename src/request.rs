use crate::models;
use itertools::Itertools;
use models::{Request, ResponseCode};

use std::collections::HashMap;

pub fn parse_request(request: String) -> Request {
    let mut request_lines = request.lines();
    let request_line = request_lines.next().unwrap();
    let (method, path_name, http_version) =
        request_line.split_whitespace().collect_tuple().unwrap();
    let mut headers = HashMap::new();
    let mut body: Option<String> = None;

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

    let req = Request {
        method: method.to_owned(),
        path_name: path_name.to_owned(),
        http_version: http_version.to_owned(),
        headers,
        body,
    };
    req
}

fn headers_to_vec(headers: Vec<String>) -> Vec<u8> {
    let mut header_string = String::new();
    for header in headers.into_iter() {
        let header = format!("{header}\r\n");
        header_string.push_str(&header);
    }
    header_string.into()
}

pub fn build_response(
    response_type: ResponseCode,
    headers: Option<Vec<String>>,
    body: Option<Vec<u8>>,
) -> Vec<u8> {
    let new_line: Vec<u8> = b"\r\n".to_vec();
    let reponse_code = response_type.as_str();
    let mut response_buffer = Vec::from(format!("HTTP/1.1 {reponse_code}"));
    response_buffer.extend_from_slice(&new_line);

    if headers.is_some() {
        let headers = headers.unwrap();
        response_buffer.append(&mut headers_to_vec(headers));
    }

    response_buffer.extend_from_slice(&new_line);

    if body.is_some() {
        let mut body = body.unwrap();
        response_buffer.append(&mut body);
    }

    response_buffer
}

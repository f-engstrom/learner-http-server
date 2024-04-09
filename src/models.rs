use std::collections::HashMap;

pub struct Request {
    pub method: String,
    pub path_name: String,
    pub http_version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

//headers
pub enum ResponseHeaders {
    ContentType,
    ContentLength,
    Server,
    SetCookie,
    Expires,
    CacheControl,
    LastModified,
    ETag,
    Pragma,
    TransferEncoding,
}
impl ResponseHeaders {
    pub fn format_header_value(&self, value: &str) -> String {
        match *self {
            ResponseHeaders::ContentLength => format!("Content-Length: {}", value),
            ResponseHeaders::ContentType => format!("Content-Type: {}", value),
            ResponseHeaders::Server => format!("Server: {}", value),
            ResponseHeaders::SetCookie => format!("Set-Cookie: {}", value),
            ResponseHeaders::Expires => format!("Expires: {}", value),
            ResponseHeaders::CacheControl => format!("Cache-Control: {}", value),
            ResponseHeaders::LastModified => format!("Last-Modified: {}", value),
            ResponseHeaders::ETag => format!("ETag: {}", value),
            ResponseHeaders::Pragma => format!("Pragma: {}", value),
            ResponseHeaders::TransferEncoding => format!("Transfer-Encoding: {}", value),
        }
    }
}

// impl ResponseHeaders {
//     pub fn as_str(&self) -> &'static str {
//         match *self {
//             ResponseHeaders::ContentLength => "Content-Length",
//             ResponseHeaders::ContentType => "Content-Type",
//             ResponseHeaders::Server => "Server",
//             ResponseHeaders::SetCookie => "Set-Cookie",
//             ResponseHeaders::Expires => "Expires",
//             ResponseHeaders::CacheControl => "Cache-Control",
//             ResponseHeaders::LastModified => "Last-Modified",
//             ResponseHeaders::ETag => "ETag",
//             ResponseHeaders::Pragma => "Pragma",
//             ResponseHeaders::TransferEncoding => "Transfer-Encoding",
//         }
//     }
// }
//responses enum

pub enum ResponseCode {
    Ok = 200,
    Created = 201,
    Accepted = 202,
    NoContent = 204,
    MovedPermanently = 301,
    Found = 302,
    NotModified = 304,
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    NotAllowed = 405,
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
}

impl ResponseCode {
    pub fn as_str(&self) -> &'static str {
        match *self {
            ResponseCode::Ok => "200 OK",
            ResponseCode::Created => "201 Created",
            ResponseCode::Accepted => "202 Accepted",
            ResponseCode::NoContent => "204 No Content",
            ResponseCode::MovedPermanently => "301 Moved Permanently",
            ResponseCode::Found => "302 Found",
            ResponseCode::NotModified => "304 Not Modified",
            ResponseCode::BadRequest => "400 Bad Request",
            ResponseCode::Unauthorized => "401 Unauthorized",
            ResponseCode::Forbidden => "403 Forbidden",
            ResponseCode::NotFound => "404 Not Found",
            ResponseCode::NotAllowed => "405 Method Not Allowed",
            ResponseCode::InternalServerError => "500 Internal Server Error",
            ResponseCode::NotImplemented => "501 Not Implemented",
            ResponseCode::BadGateway => "502 Bad Gateway",
            ResponseCode::ServiceUnavailable => "503 Service Unavailable",
        }
    }
}

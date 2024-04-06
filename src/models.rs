use std::collections::HashMap;

pub struct Request {
    pub method: String,
    pub path_name: String,
    pub http_version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

//headers
//responses enum

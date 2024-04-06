// Uncomment this block to pass the first stage
use crate::connection::handle_connection;
use std::{io::Write, net::TcpListener, thread};

pub mod connection;
pub mod models;
pub mod request;
pub mod route;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                thread::spawn(move || {
                    let msg = handle_connection(&stream);
                    stream.write(&msg);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

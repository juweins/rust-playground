/* This file contains the HTTP Server struct implementation.
// On startup, main.rs calls the run() method of this file to
// bind a TCPlistener on the IP-Address:Port
// 
// IP-Adress is passed during creation of HTTPServer instance.
*/


use crate::http::{Response, Request, StatusCode};
use std::convert::TryFrom;
use std::net::TcpListener;
use std::io::{Read, Write};

pub struct HTTPServer {
    ip_address: String,
}

impl HTTPServer {
    // Self is a special keyword, which is an alias for the structs' name
    pub fn new(ip_address: String) -> Self {
        Self { ip_address }
    }

    // This is a method which uses self reference
    pub fn run(self) {
        print!("Server listening on {}", self.ip_address);
        let listener = TcpListener::bind(&self.ip_address).unwrap();

        // Infinite looping to keep server listening 
        loop {
            // accept valid, incoming TCP strems
            match listener.accept() {
                Ok((mut stream, address)) => {
                    let mut buffer = [0u8; 1024];
                    match stream.read(&mut buffer){
                        Ok(_) => {
                            println!("Received request: {}", String::from_utf8_lossy(&buffer));

                            // parse buffer to Request
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                    // Serving hardcoded html as a placeholder
                                    let response = Response::new(StatusCode::Ok, Some("<h1>Hello RustWorld!</h1>".to_string()));
                                    write!(stream, "{}", response);
                                }
                                Err(e) => {
                                    println!("Failed parsing request: {}", e)
                                }

                            }

                        }
                        Err(e) => {
                            println!("Failed reading stream: {}", e)
                        }
                    }
                }
                Err(e) => {
                    println!("Failed {}", e);
                }
            }
        }
    }
}

#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]


use http::Method;
use http::Request;
use server::HTTPServer;

mod server;
mod http;

fn main() {
    // The server should listen on localhost:8080 by default
    let server = HTTPServer::new("127.0.0.1:8080".to_string());
    server.run();
}

#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// builtin
use std::env;
// custom
use http::Method;
use http::Request;
use server::HTTPServer;
use website_handler::WebsiteHandler;


mod server;
mod http;
mod website_handler;

fn main() {

    // using the cargo_manifest_dir will return the pwd for default_path
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);

    // The server should listen on localhost:8080 by default
    let server = HTTPServer::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path));
}

use super::server::Handler;
use super::http::{Response, Request, StatusCode, Method};

use std::fs;

pub struct WebsiteHandler{
    public_path: String
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self {public_path}
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);

        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory traversal attack attempted: {}", file_path);
                    None
                }
            }
            Err(_) => None
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None),
                }
            },
            _ => Response::new(StatusCode::NotFound, None),

        
            Method::POST => todo!(),
            Method::DELETE => todo!(),
            Method::CONNECT => todo!(),
            Method::HEAD => todo!(),
            Method::OPTIONS => todo!(),
            Method::PATCH => todo!(),
            Method::PUT => todo!(),
            Method::TRACE => todo!(),
        }
    }
}
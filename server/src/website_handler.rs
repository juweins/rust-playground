use super::server::Handler;
use super::http::{Response, Request, StatusCode, Method};

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, Some("<h1>Welcome to my Rust server<h1>".to_string())),
                "/main" => Response::new(StatusCode::Ok, Some("<h1>This is a path destination<h1>".to_string())),
                _ => Response::new(StatusCode::NotFound, None),
            },

            _ => Response::new(StatusCode::NotFound, None),

        }

    }
}
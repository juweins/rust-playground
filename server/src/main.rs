use http::request::Request;
use server::HTTPServer;

fn main() {
    // The server should listen on localhost:8080 by default
    let server = HTTPServer::new("127.0.0.1:8080".to_string());
    server.run();
}

mod server {
    pub struct HTTPServer {
        ip_address: String,
    }

    impl HTTPServer {
        // Self is a special keyword, which is an alias for the structs' name
        pub fn new(ip_address: String) -> Self {
            Self { ip_address }
        }

        // This is a method which uses self reference
        pub fn run(self) {}
    }
}

// This is a module with submodules
mod http {
    pub mod request {
        use super::method::Method;
        pub struct Request {
            path: String,
            method: Method,
            query_string: Option<String>,
        }
    }

    pub mod method {
        pub enum Method {
            GET,
            POST,
            DELETE,
            CONNECT,
            HEAD,
            OPTIONS,
            PATCH,
            PUT,
            TRACE,
        }
    }
}

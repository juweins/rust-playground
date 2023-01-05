fn main() {

    // The server should listen on localhost:8080 by default
    let server = HTTPServer::new("127.0.0.1:8080".to_string());
    server.run();

}

struct HTTPServer {
    ip_address: String,
}

impl HTTPServer {

    // Self is a special keyword, which is an alias for the structs' name
    fn new(ip_address: String) -> Self{
        Self{
            ip_address
        }
    }

    // This is a method which uses self reference
    fn run(self) {

    }
}

// Options are a way to deal with the absence of a value.
// (in our case, if we would pass no query string)
struct HTTPRequest {
    path: String,
    method: Method,
    query_string: Option<String>,
}

enum Method {
    GET,
    POST,
    DELETE,
    CONNECT,
    HEAD,
    OPTIONS,
    PATCH,
    PUT,
    TRACE
}
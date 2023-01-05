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
